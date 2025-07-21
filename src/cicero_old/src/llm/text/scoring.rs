
use super::PreProcessor;

// Score similarity of two names
pub fn name_similar(needle: &str, haystack: &str) -> f32 {

    // Pre-process names
    let processor = PreProcessor::new(Default::default());
    let needle_words = processor.scrub_name(&needle);
    let haystack_str = processor.scrub_name(&haystack).join(" ").clone();

    let count = needle_words.into_iter().filter(|w| haystack_str.contains(*&w)).collect::<Vec<_>>().len();

    println!("Count: {}", count);

    let mut score: f32 = 6.4;
    score
}


