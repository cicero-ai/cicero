
use std::path::Path;
use atlas_http::HttpClient;
use ndarray::{Axis, Ix2};
use falcon_cli::*;
use lazy_static::lazy_static;
use std::sync::Mutex;
use ort::{
    execution_providers::CUDAExecutionProvider,
    session::{Session, builder::GraphOptimizationLevel},
    value::TensorRef
};
use tokenizers::Tokenizer;
use cicero::utils::sys;
use crate::{Error, CONFIG};

lazy_static! {
    pub static ref ONNX_SESSION: Mutex<Session> = load_onnx_model();
}


type Encodings = Vec<(Vec<i64>, Vec<i64>, Vec<i64>)>;

const MODEL_NAME: &'static str = "all-mpnet-base-v2";

/// Generate sentence embeddings
pub fn generate(inputs: &Vec<String>) -> Result<Vec<Vec<f32>>, Error> {

    // Encode inputs
    let encodings = encode_inputs(&inputs)?;

    // Get token IDs & mask as a flattened array.
    let padded_token_length = encodings[0].0.len();
    let input_ids: Vec<i64> = encodings.iter().flat_map(|(ids, _, _)| ids.iter().copied()).collect();
    let mask: Vec<i64> = encodings.iter().flat_map(|(_, mask, _)| mask.iter().copied()).collect();

    // Convert our flattened arrays into 2-dimensional tensors of shape [N, L].
    let a_input_ids = TensorRef::from_array_view(([inputs.len(), padded_token_length], &*input_ids))
        .map_err(|e| Error::Onnx( format!("Unable to convert input_ids to 2d array, error: {}", e)) )?;
    let a_mask = TensorRef::from_array_view(([inputs.len(), padded_token_length], &*mask))
        .map_err(|e| Error::Onnx( format!("Unable to convert attention masks to 2d array, error: {}", e)) )?;

    // Run the model.
    let mut session = ONNX_SESSION.lock().map_err(|e| Error::Onnx(format!("Failed to lock session: {}", e)))?;
    let outputs = session.run(ort::inputs![a_input_ids, a_mask])
        .map_err(|e| Error::Onnx( format!("Unable to run inference on onnx model, error: {}", e)) )?;

    // Extract embeddings tensor and convert it to a strongly-typed 2-dimensional array.
    let embedding_tensor = outputs["last_hidden_state"].try_extract_tensor::<f32>()
        .map_err(|e| Error::Onnx( format!("Unable to extract tensor from onnx output, error: {}", e)) )?;
    let shape = embedding_tensor.shape();

    // Mean pool to [batch_size, hidden_size]
    let batch_size = shape[0];
    let hidden_size = shape[2];
    let seq_len = shape[1];

    let mut res = Vec::with_capacity(batch_size);
    for batch_idx in 0..batch_size {
        let mut embedding = Vec::with_capacity(hidden_size);
        for dim in 0..hidden_size {
            let (mut sum, mut count) = (0.0, 0);
            for seq in 0..seq_len {
                // Use attention_mask here if available to skip padding
                sum += embedding_tensor[[batch_idx, seq, dim]];
                count += 1;
            }
            embedding.push(if count > 0 { sum / count as f32 } else { 0.0 });
        }
        res.push(embedding);

    }

    Ok(res)
}

/// Load onnx model
fn load_onnx_model() -> Mutex<Session> {

    // Download model, if needed
    let (model_path, tokenizer_path) = download().map_err(|e| error(format!("Unable to download onnx model, error: {}", e))).unwrap();

    // Create the ONNX Runtime environment, enabling CUDA execution providers for all sessions created in this process.
    ort::init()
        .with_name("sbert")
        .with_execution_providers([CUDAExecutionProvider::default().build()])
        .commit()
        .map_err(|e| error(format!("Unable to initialize onnx runtime, error: {}", e))).unwrap();

    // Load our model
    let mut session: Session = Session::builder()
        .map_err(|e| error(format!("Unable to load onnx session, error: {}", e))).unwrap()
        .with_optimization_level(GraphOptimizationLevel::Level1)
            .map_err(|e| error(format!("Unable to load onnx session, error: {}", e))).unwrap()
        .with_intra_threads(1)
            .map_err(|e| error(format!("Unable to load onnx session, error: {}", e))).unwrap()
        .commit_from_file(&model_path)
            .map_err(|e| error(format!("Unable to load onnx session, error: {}", e))).unwrap();

        Mutex::new(session)
}

fn error(msg: String) {
    cli_error!("error: {}", msg);
    std::process::exit(1);
}

// Download model
fn download() -> Result<(String, String), Error> {

    // Set paths
    let model_path = format!("{}/models/{}/model.onnx", sys::get_datadir(), MODEL_NAME);
    let tokenizer_path = format!("{}/models/{}/tokenizer.json", sys::get_datadir(), MODEL_NAME);

    // Check if exists
    if Path::new(&model_path).exists() {
        return Ok((model_path, tokenizer_path));
    }
    sys::prepare_parent_dir(&model_path);
    cli_info!("Downloading onnx model for embeddings, please be patient...");

    // Download
    let mut http = HttpClient::builder().browser().build_sync();
    http.download(&format!("https://cicero.sh/downloads/{}/model.onnx", MODEL_NAME), &model_path)
        .map_err(|e| Error::Onnx( format!("Unable to download onnx model: {}", e)) )?;
    http.download(&format!("https://cicero.sh/downloads/{}/tokenizer.json", MODEL_NAME), &tokenizer_path)
        .map_err(|e| Error::Onnx( format!("Unable to download onnx model: {}", e)) )?;

    Ok((model_path, tokenizer_path))
}

// Encode inputs
fn encode_inputs(inputs: &Vec<String>) -> Result<Encodings, Error> {

    // Load tokenizer
    let tokenizer_path = format!("{}/models/{}/tokenizer.json", sys::get_datadir(), MODEL_NAME);
    let tokenizer = Tokenizer::from_file(Path::new(&tokenizer_path))
        .map_err(|e| Error::Onnx( format!("Unable to load tokenizer, error: {}", e)) )?;


    // Encode all inputs without padding (we’ll handle it per chunk)
    let encodings = tokenizer.encode_batch(inputs.clone(), false)
        .map_err(|e| Error::Onnx(format!("Unable to tokenize inputs, error: {}", e)))?;

    let mut result = Vec::new();
    const MAX_LEN: usize = 128;

    for encoding in encodings.iter() {
        let ids = encoding.get_ids();
        let mask = encoding.get_attention_mask();
        let type_ids = encoding.get_type_ids();

        // Chunk each input’s tokens
        for chunk in ids.chunks(MAX_LEN) {
            let chunk_len = chunk.len();
            // Pad or truncate to exactly 128
            let mut chunk_ids: Vec<i64> = chunk.iter().map(|&x| x as i64).collect();
            let mut chunk_mask: Vec<i64> = mask[..chunk_len].iter().map(|&x| x as i64).collect();
            let mut chunk_type_ids: Vec<i64> = type_ids[..chunk_len].iter().map(|&x| x as i64).collect();

            if chunk_len < MAX_LEN {
                chunk_ids.resize(MAX_LEN, 0); // Pad with 0 (pad_token_id)
                chunk_mask.resize(MAX_LEN, 0); // Mask padding as 0
                chunk_type_ids.resize(MAX_LEN, 0); // Type IDs typically 0 for padding
            }

            result.push((chunk_ids, chunk_mask, chunk_type_ids));
        }
    }

    Ok(result)
}


