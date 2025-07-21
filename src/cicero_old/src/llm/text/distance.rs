
use std::iter::zip;
use std::f32::consts::PI;

// Cosine Distance
pub fn cosine_distance(v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
    assert_eq!(v1.len(), v2.len(), "Vectors must be of the same length");

    let dot_product: f32 = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum();

    let v1_norm: f32 = (v1.iter().map(|x| x.powi(2)).sum::<f32>()).sqrt();
    let v2_norm: f32 = (v2.iter().map(|x| x.powi(2)).sum::<f32>()).sqrt();

    1.0 - dot_product / (v1_norm * v2_norm)
}

// Get euclidean distance
pub fn euclidean_distance(vec1: &Vec<f32>, vec2: &Vec<f32>) -> f32 {

    vec1.iter()
        .zip(vec2.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f32>()
        .sqrt()
}

// Manhattan Distance
pub fn manhattan_distance(v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
    assert_eq!(v1.len(), v2.len(), "Vectors must be of the same length");

    v1.iter()
        .zip(v2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}



