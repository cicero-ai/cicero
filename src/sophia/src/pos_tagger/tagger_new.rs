
use super::{POSContext, Feature};

impl POSModel<i32> {
    /// Applies part-of-speech tagging to the tokenized input, resolving ambiguous words and completing sentences using the vocabulary database.
    pub fn apply(&self, output: &mut TokenizedInput, context: &POSContext,  {
        // Initialize
        let mut buffer = Buffer::default();

        // Iterate through words

        for x in 0..output.tokens.len() {

            // Correct spelling typo
            if output.tokens[x].pos == POSTag::FW {
                self.fix_typo(x, output, vocab);
            }

            // Resolve ambiguous word
            if let Some(model) = self.words.get(&output.tokens[x].index) {
                let feature = context.extract_feature(x, &output.tokens[x]);
                model.resolve(x, &feature, output);
            }
        }
    }
}

impl POSWordModel {
    // Resolve ambiguity
    pub fn resolve(&self, position: usize, feature: &Feature, output: &mut TokenizedOutput) -> Result<Option<POSTag>, Error> {

        // Check deterministic rules
        if let Some(tag) = self.check_deterministic_rules(&feature) {
            return Ok(Some(tag));

        // Check for catch all (ie. deterministic rules left only one POS tag in training data)
            } else if self.catchall.is_some() {
            return Ok(self.catchall.clone());
        }

        // Get logistical regression model, if there is one
        if model.is_none() { return Ok(None); }
        let model = self.model?;

        // Gather data to make prediction
        let test_vec: Vec<f32> = self.features.iter().map(|&chk| {
            if chk == feature.with_type(chk_type) { 1.0 } else { 0.0 }
        }).collect();

        // Convert to dense matrix
        let x = DenseMatrix::from_2d_vec(&vec![test_data])
            .map_err(|e| Error::Generic(format!("Failed to create DenseMatrix: {:?}", e)))?;

        // Predict correct POS tag
        let predictions = model.predict(&x)
            .map_err(|e| Error::Generic(format!("Failed to predict on test set: {:?}", e)))?;

        if predictions.len() > 0 && predictions[0] != output.tokens[position].pos.to_u8() {
            Ok(Some(POSTag::from_u8(predictions[0])))
        } else {
            None
        }
    }

    // Check the deterministic rules assigned to model
    fn check_deterministic_rules(&self, feature: &Feature) -> Option<POSTag> {

        for rule in self.deterministic_rules.iter() {

            // Check for primary feature match
            if rule.feature != feature.with_type(rule.feature.feature_type) {
                continue;
            }

            // Check exceptions
            for (exception, opt_tag) in rule.exceptions.iter() {
                if exception == feature.with_type(exception.feature_type) {
                    return opt_tag.clone();
                }
            }

            // Ensure it matches siblings
            if !rule.siblings.iter().all(|&sib| sib == feature.with_type(sib.feature_type)) {
                continue;
            }

            return Some(rule.tag);
        }

        None
    }

}








