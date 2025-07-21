
use atlas_http::{HttpResponse, HttpRequest};
use crate::error::Error;
use crate::server::api;
use crate::server::apollo::structs::{TextGenerationRequest, QuestionAnswerRequest};
use super::ApolloServer;
use log::debug;

/// Route and handle the request appropriately
pub fn handle(apollo: &mut ApolloServer, parts: &Vec<String>, req: &HttpRequest) -> HttpResponse {

    // Get body
    let body = format!("{}", String::from_utf8_lossy(&req.body.get_raw().clone()));
    debug!("Handling NLP request for {}", parts[2]);

    match parts[2].as_str() {
        "summarize" => summarize(apollo, &body.as_str()),
        "text_generation" => text_generation(apollo, &body.as_str()),
        "question_answer" => question_answer(apollo, &body.as_str()),
        "sentence_embeddings" => sentence_embeddings(apollo, &body.as_str()),
        "pos_tagging" => pos_tagging(apollo, &body.as_str()),
        _ => api::response(404, "No endpoint at this location", String::new())
    }
}

/// Summarize text
fn summarize(apollo: &mut ApolloServer, req_body: &str) -> HttpResponse {

    // Decode json
    let inputs: Vec<String> = match serde_json::from_str(&req_body) {
        Ok(r) => r,
        Err(_) => return api::response(400, "Malformed JSON within request, did not consist of one-dimensional array of text inputs..", String::new())
    };
        debug!("Decoded {} summarization inputs, sending to LLM", inputs.len());

    // Summarize
    let output = match apollo.summarizer.summarize(&inputs) {
        Ok(r) => r,
        Err(e) => return api::response(500, format!("Received error from summarizer pipeline: {}", e.to_string()).as_str(), String::new())
    };
    debug!("Successfully completed summarization, returning results to Apollo");

    api::response(200, "", output)
}

/// Text generation
fn text_generation(apollo: &mut ApolloServer, req_body: &str) -> HttpResponse {

    // Decode json
    let request: TextGenerationRequest = match serde_json::from_str(&req_body) {
        Ok(r) => r,
        Err(_) => return api::response(400, "Malformed JSON within request, please check documentation for proper JSON format of text generation pipeline...", String::new())
    };
        debug!("Decoded {} text generation inputs, sending to LLM", request.input.len());

    // Generate text
    let output = match apollo.text_generation.generate(&request.input, &request.prefix.as_str()) {
        Ok(r) => r,
        Err(e) => return api::response(500, format!("Received error from text generation pipeline: {}", e.to_string()).as_str(), String::new())
    };
    debug!("Successfully completed text generation, returning results to Apollo");

    api::response(200, "", output)
}

/// Question answer
fn question_answer(apollo: &mut ApolloServer, req_body: &str) -> HttpResponse {

    // Decode json
    let request: QuestionAnswerRequest = match serde_json::from_str(&req_body) {
        Ok(r) => r,
        Err(_) => return api::response(400, "Malformed JSON within request, please check documentation for proper JSON format of question answer pipeline...", String::new())
    };
        debug!("Decoded {} question answer inputs, sending to LLM", request.input.len());

    // Generate text
    let output = match apollo.question_answer.ask(&request.input, &request.context.as_str()) {
        Ok(r) => r,
        Err(e) => return api::response(500, format!("Received error from question answer pipeline: {}", e.to_string()).as_str(), String::new())
    };
    debug!("Successfully completed question answer, returning results to Apollo");

    api::response(200, "", output)
}

/// Generate sentence embeddings
fn sentence_embeddings(apollo: &mut ApolloServer, req_body: &str) -> HttpResponse {

    // Decode json
    let inputs: Vec<String> = match serde_json::from_str(&req_body) {
        Ok(r) => r,
        Err(_) => return api::response(400, "Malformed JSON within request, did not consist of one-dimensional array of text inputs..", String::new())
    };
        debug!("Decoded {} sentence embedding inputs, sending to LLM", inputs.len());

    // Generate embeddings
    let output = match apollo.sentence_embeddings.encode(&inputs) {
        Ok(r) => r,
        Err(e) => return api::response(500, format!("Received error from sentence embeddings pipeline: {}", e.to_string()).as_str(), String::new())
    };
    debug!("Successfully completed encoding sentence embeddings, returning results to Apollo");

    api::response(200, "", output)
}

/// POS Tagging
fn pos_tagging(apollo: &mut ApolloServer, req_body: &str) -> HttpResponse {

    // Decode json
    let inputs: Vec<String> = match serde_json::from_str(&req_body) {
        Ok(r) => r,
        Err(_) => return api::response(400, "Malformed JSON within request, did not consist of one-dimensional array of text inputs..", String::new())
    };
        debug!("Decoded {} POS tagging inputs, sending to LLM", inputs.len());

    // Summarize
    let output = match apollo.pos_tagging.predict(&inputs) {
        Ok(r) => r,
        Err(e) => return api::response(500, format!("Received error from POS pipeline: {}", e.to_string()).as_str(), String::new())
    };
    debug!("Successfully completed summarization, returning results to Apollo");

    api::response(200, "", output)
}


