use super::{POSTag, POSTagger, POSTaggerLayer, POSTaggerScores};
use crate::tokenizer::TokenizedInput;
use crate::vocab::f8::f8;
use crate::vocab::VocabDatabase;
use std::collections::HashMap;
use std::fmt;

const TAG_SCORE: f32 = 0.3;
const BEFORE_BIGRAM_SCORE: f32 = 0.6;
const AFTER_BIGRAM_SCORE: f32 = 0.4;
const BEFORE_WEIGHTS: [f32; 4] = [0.7, 0.4, 0.3, 0.2];
const AFTER_WEIGHTS: [f32; 2] = [0.85, 0.25];
const INITIAL_WEIGHTS: [f32; 2] = [0.65, 0.35];

/// A buffer for storing part-of-speech tags and resolved word scores during tagging.
#[derive(Default)]
struct Buffer {
    pub start: usize,
    pub tags: Vec<POSTag>,
    pub words: Vec<ResolvedScore>,
}

/// Represents a resolved score for a word, including its position, assigned tag, maximum score, and score distribution for possible tags.
#[derive(Default)]
struct ResolvedScore {
    pub position: usize,
    pub tag: POSTag,
    pub max_score: f32,
    pub scores: HashMap<POSTag, f32>,
}

impl POSTagger<f8, i32> {
    /// Applies part-of-speech tagging to the tokenized input, resolving ambiguous words and completing sentences using the vocabulary database.
    pub fn apply(&self, output: &mut TokenizedInput, vocab: &VocabDatabase) {
        // Initialize
        let mut buffer = Buffer::default();

        // Iterate through words
        for x in 0..output.tokens.len() {
            // Correct spelling typo
            if output.tokens[x].pos == POSTag::FW {
                self.fix_typo(x, output, &vocab);
            }

            // Resolve ambiguous word
            if let Some(layer) = self.tag2word.get(&output.tokens[x].index) {
                let score = layer.resolve(&buffer);
                buffer.tags.push(score.tag);
                buffer.words.push(score);

            // Complete sentence, if stopper
            } else if output.tokens[x].pos == POSTag::SS {
                self.complete_sentence(&mut buffer, output, vocab);
                buffer = Buffer::new(x + 1);
            } else {
                buffer.tags.push(output.tokens[x].pos);
            }
        }

        // Complete buffer
        if !buffer.words.is_empty() {
            self.complete_sentence(&mut buffer, output, vocab);
        }
    }

    /// Completes a sentence by verifying and updating POS tags for ambiguous words in the buffer, applying changes to the tokenized input.
    fn complete_sentence(
        &self,
        buffer: &mut Buffer,
        output: &mut TokenizedInput,
        vocab: &VocabDatabase,
    ) {
        if buffer.words.is_empty() {
            return;
        }

        // Verify by checking following tags of ambiguous words
        for x in 0..buffer.words.len() {
            let buffer_pos = buffer.start + buffer.words[x].position;
            let layer = self.tag2word.get(&output.tokens[buffer_pos].index).unwrap();

            let tag = match layer.after.verify_score(x, buffer) {
                Some(r) => r,
                None => buffer.words[x].tag,
            };
            if tag == output.tokens[buffer.words[x].position].pos {
                continue;
            }

            // Update POS tag
            if let Some(new_token) = output.tokens[buffer_pos].update_pos(tag, vocab) {
                output.tokens[buffer_pos] = new_token;
            }
        }
    }

    /// Try to fix a spelling typo
    fn fix_typo(&self, position: usize, output: &mut TokenizedInput, vocab: &VocabDatabase) {
        let start = position.saturating_sub(8);
        let end = (position + 4).min(output.tokens.len() - 1);

        // Predict tag
        let buffer =
            output.tokens[start..end].iter().map(|token| token.pos).collect::<Vec<POSTag>>();
        let buffer_pos = if position >= 8 { 8 } else { position };
        let new_tag = self.tag2tag.predict(buffer_pos, &buffer);

        // Check spell checker
        if let Some(correct) = vocab.preprocess.spellchecker.try_correct(
            &output.tokens[position].word,
            new_tag,
            &vocab.words,
        ) {
            output.tokens[position] = correct;
        }
    }
}

impl POSTaggerLayer<f8> {
    /// Predicts the next token based on both, preceeding and following contexts at once.
    /// Intended when full context is available vs. standard sequential POS tagging
    fn predict(&self, position: usize, buffer: &Vec<POSTag>) -> POSTag {
        if position >= buffer.len() {
            return POSTag::FW;
        }

        // Check for single buffer
        if buffer.len() == 1 || position + 1 == buffer.len() {
            return buffer[position];
        }

        // Initialize variables
        let (mut max_tag, mut max_score) = (buffer[position].clone(), 0.0);
        let mut scores: HashMap<POSTag, f32> = HashMap::new();

        // Initial
        if position == 0 {
            let tags = buffer[1..].to_vec().into_iter().take(4).collect::<Vec<POSTag>>();
            let bigram_scores = self.initial.calculate_bigram_score(&tags, &INITIAL_WEIGHTS);
            for (tag, score) in bigram_scores.iter() {
                if *score > max_score {
                    max_tag = *tag;
                    max_score = *score;
                }
            }

            if max_score > 0.0 {
                return max_tag;
            }
        }

        // Before scores
        if position > 0 {
            let start = position.saturating_sub(8);
            let before_tags =
                buffer[start..position].to_vec().into_iter().rev().collect::<Vec<POSTag>>();
            let bigram_scores = self.before.calculate_bigram_score(&before_tags, &BEFORE_WEIGHTS);

            // Apply bigram weight
            for (tag, score) in bigram_scores.iter() {
                scores.insert(*tag, *score * BEFORE_BIGRAM_SCORE);
            }
        }

        // Get after scores
        if position > buffer.len() {
            let tags = buffer[position..].to_vec().into_iter().take(4).collect::<Vec<POSTag>>();
            let bigram_scores = self.after.calculate_bigram_score(&tags, &INITIAL_WEIGHTS);

            for (tag, score) in bigram_scores.iter() {
                *scores.entry(*tag).or_insert(0.0) += score * AFTER_BIGRAM_SCORE;
            }
        }

        // Check bigram scores
        for (tag, score) in scores.iter() {
            if *score > max_score {
                max_tag = *tag;
                max_score = *score;
            }
        }

        max_tag
    }

    /// Resolves the POS tag for a word by calculating bigram scores from previous tags and selecting the highest-scoring tag.
    fn resolve(&self, buffer: &Buffer) -> ResolvedScore {
        // Get bigram scores, if we have previous words
        let mut bigram_scores: HashMap<POSTag, f32> = HashMap::new();
        if !buffer.tags.is_empty() {
            let tags = buffer.tags.clone().into_iter().rev().take(8).collect::<Vec<POSTag>>();
            bigram_scores = self.before.calculate_bigram_score(&tags, &BEFORE_WEIGHTS)
        }
        let mut res = ResolvedScore::new(buffer.tags.len());

        // Get highest score
        let (mut max_tag, mut max_score) = (POSTag::FW, 0.0);
        for (tag, score_f8) in self.tags.iter() {
            let score = match bigram_scores.get(tag) {
                Some(bigram_score) => {
                    (score_f8.to_f32() * TAG_SCORE) + (bigram_score * BEFORE_BIGRAM_SCORE)
                }
                None => score_f8.to_f32() * TAG_SCORE,
            };
            res.scores.insert(*tag, score);

            if score > max_score {
                max_score = score;
                max_tag = *tag;
            }
        }

        // Set results
        res.max_score = max_score;
        res.tag = max_tag;

        res
    }
}

impl POSTaggerScores<f8> {
    /// Calculates bigram scores for a sequence of tags, weighting them according to provided weights and averaging scores per tag.
    pub fn calculate_bigram_score(
        &self,
        buffer: &Vec<POSTag>,
        weights: &[f32],
    ) -> HashMap<POSTag, f32> {
        // Initialize
        let mut scores: HashMap<POSTag, Vec<f32>> = HashMap::new();

        // Iterate through bigrams
        for (offset, chunk) in buffer.chunks(2).enumerate() {
            let bigram = if chunk.len() == 1 {
                (chunk[0].to_u8() << 6) as u16
            } else {
                ((chunk[0].to_u8() << 6) | chunk[1].to_u8()) as u16
            };

            if let Some(score_map) = self.bigrams[offset].0.get(&bigram) {
                for (tag, score) in score_map.iter() {
                    scores.entry(*tag).or_default().push(score.to_f32() * weights[offset]);
                }
            }
        }
        if scores.is_empty() {
            return HashMap::new();
        }

        // Create results
        let mut res: HashMap<POSTag, f32> = HashMap::new();
        for (tag, scores_vec) in scores.iter() {
            let score = scores_vec.clone().into_iter().sum::<f32>() / scores_vec.len() as f32;
            res.insert(*tag, score);
        }

        res
    }
    /// Verifies the score of a word by calculating bigram scores for following tags, updating the tag if a higher score is found.
    fn verify_score(&self, word_x: usize, buffer: &Buffer) -> Option<POSTag> {
        if buffer.tags.len() < (buffer.words[word_x].position + 2) {
            return None;
        }

        // Get next tags
        let end = (buffer.words[word_x].position + 5).min(buffer.tags.len());
        let tags = buffer.tags[(buffer.words[word_x].position + 1)..end].to_vec();

        // Calculate bigram score
        let scores = self.calculate_bigram_score(&tags, &AFTER_WEIGHTS);

        // Get new highest score
        let (mut max_tag, mut max_score) =
            (buffer.words[word_x].tag, buffer.words[word_x].max_score);
        for (tag, score) in scores.iter() {
            let mut cur_score: f32 = match buffer.words[word_x].scores.get(tag) {
                Some(r) => *r,
                None => continue,
            };
            cur_score += score * AFTER_BIGRAM_SCORE;

            // Check score
            if cur_score > max_score {
                max_tag = *tag;
                max_score = cur_score;
            }
        }

        // Return
        if max_tag != buffer.words[word_x].tag {
            Some(max_tag)
        } else {
            None
        }
    }
}

impl ResolvedScore {
    /// Creates a new ResolvedScore instance with the specified position and default values for other fields.
    pub fn new(position: usize) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }
}

impl Buffer {
    pub fn new(start: usize) -> Self {
        Self {
            start,
            ..Default::default()
        }
    }
}

impl fmt::Debug for ResolvedScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scores_vec = self
            .scores
            .iter()
            .map(|(tag, score)| format!("({} {})", tag.to_str(), score))
            .collect::<Vec<String>>();
        write!(
            f,
            "Resolved Score:  pos {} tag {} score {}\n",
            self.position,
            self.tag.to_str(),
            self.max_score
        )?;
        write!(f, "    scores: {}", scores_vec.join(" ").to_string())
    }
}
