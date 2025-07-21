
use cicero_sdk::api::{Cicero, CiceroSTD, CiceroNLP};
use super::{CiceroAPI_STD, CiceroAPI_NLP};

pub struct CiceroAPI {
    std: Box<dyn CiceroSTD>,
    nlp: Box<dyn CiceroNLP>
}

impl CiceroAPI {

    pub fn new() -> Box<dyn Cicero> {
        Box::new(Self {
            std: Box::new(CiceroAPI_STD::new()),
            nlp: Box::new(CiceroAPI_NLP::new())
        })
    }
}


impl Cicero for CiceroAPI {

    fn std(&self) -> &Box<dyn CiceroSTD> {
        &self.std
    }

    fn nlp(&self) -> &Box<dyn CiceroNLP> {
        &self.nlp
    }

}

