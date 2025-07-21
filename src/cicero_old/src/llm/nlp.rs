

use std::collections::HashMap;
use crate::server::apollo::structs::{TextGenerationRequest, QuestionAnswerRequest};
use serde::{Serialize, Deserialize};
use crate::error::Error;
use crate::utils::api_client;
use std::fmt::Display;
use log::debug;

/// Generate sentence embeddings
pub fn sentence_embeddings(input: &Vec<String>) -> Result<Vec<Vec<f32>>, Error> {
    debug!("Sending {} chunks to Apollo for sentence embeddings generation", input.len());

    // Go through input in batches
        let mut results = Vec::new();
    for batch in input.chunks(5) {
        let res: Vec<Vec<f32>> = api_client::send_json::<Vec<Vec<f32>>, Vec<String>>("v1/nlp/sentence_embeddings", "POST", batch.to_vec())?;
        results.extend(res);
    }
    Ok(results)
}

/// POS Tagging
pub fn pos_tagging(input: &Vec<String>) -> Result<Vec<Vec<POSTag>>, Error> {

    // Go through input in batches
        let mut results = Vec::new();
    for batch in input.chunks(20) {
        debug!("Sending request to apollo for POS tagging");
        let res: Vec<Vec<POSTag>> = api_client::send_json::<Vec<Vec<POSTag>>, Vec<String>>("v1/nlp/pos_tagging", "POST", batch.to_vec())?;
        results.extend(res);
        debug!("Received POS tagging results, continuing to next batch");
    }

    Ok(results)
}


