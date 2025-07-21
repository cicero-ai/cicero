
use tokio::runtime::Runtime;
use tokio::net::TcpStream;
use llm_api_rs::core::{ChatCompletionRequest, ChatMessage};
use crate::llm::models::LlmProfile;

#[derive(Serialize, Deserialize)]
pub struct OllamaWord {
    pub response: String,
    pub done: bool
}

    /// Send to Ollama
async fn send_streamed(prompt: &String, profile: &LlmProfile, stream: &mut std::net::TcpStream) -> Result<String, Error> {





    // Start ollama
    let ollama = Ollama::default();
    let mut res_stream = ollama.generate_stream(GenerationRequest::new(model_name.to_string(), prompt.to_string())).await.unwrap();

    // Send success http status header
    stream.write("HTTP/1.1 200 OK\n\n".as_bytes()).unwrap();
    stream.flush().unwrap();

    // Send request to ollama
    let mut stdout = tokio::io::stdout();
    while let Some(res) = res_stream.next().await {
        let responses = res.unwrap();
        for resp in responses {
            response.push_str(&resp.response.as_str());
            let word = OllamaWord { response: resp.response.to_string(), done: resp.done };
            let json = format!("{}\n", serde_json::to_string(&word).unwrap());

            stream.write(json.as_bytes()).unwrap();
            stream.flush().unwrap();

            stdout.write(resp.response.as_bytes()).await.unwrap();
            stdout.flush().await.unwrap();
        }
    }

    Ok(response)
}


