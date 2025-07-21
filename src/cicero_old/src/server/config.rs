
use crate::server::apollo::manager::setup;
use std::path::Path;
use std::fs;
use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};
use crate::utils::sys;
use log::error;

#[derive(Debug, Serialize, Deserialize)]
pub struct CiceroServerConfig {
    pub general: ConfigGeneral,
    pub daemons: ConfigDaemons,
    pub ml: ConfigML,
    pub items_per_batch: ConfigItemsPerBatch,
    pub models: ConfigModels,
    pub api_keys: HashMap<String, String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigGeneral {
    pub mode: String,
    pub libdir: String,
    pub plugin_dev_dir: String,
    pub language: String,
    pub agree_share: bool,
    pub num_threads: usize,
    pub use_gpu: bool,
    pub network_mode: String,
    pub api_client: String,
    pub ollama_url: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigDaemons {
    pub apollo: (String, u16),
    pub helios: (String, u16),
    pub echo: (String, u16)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigML {
    pub embedding_chunk_maxlength: usize,
    pub embedding_search_threshold: f32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigItemsPerBatch {
    pub summarization: usize,
    pub sentence_embeddings: usize
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigModels {
    #[serde(default)]
    pub autoload: String,
    pub summarization: String,
    pub text_generation: String,
    pub question_answer: String,
    pub ner: String,
    pub keyword_extraction: String,
    pub zero_shot_classification: String,
    pub pos_tagging: String,
    pub sentiment_analysis: String,
    pub sentence_embeddings: String,
    pub sequence_classification: String,
    pub token_classification: String,

}


impl CiceroServerConfig {

    pub fn new() -> Self {

        // Setup datadir
        let config_file = format!("{}/config/server.yml", sys::get_datadir());
        if !Path::new(&config_file).exists() {
            setup::run();
        }

        // Load yaml file
        let yaml_code = fs::read_to_string(&config_file).unwrap();
        let mut config: CiceroServerConfig = match serde_yaml::from_str(&yaml_code) {
            Ok(r) => r,
            Err(e) => {
                error!("Unable to properly deserialize the configuration file at {}, error: {}", config_file, e.to_string());
                std::process::exit(1);
            }
        };

        config
    }

    /// Save configuratoni
    pub fn save(&self) {

        let config_file = format!("{}/config/server.yml", sys::get_datadir());
        let yaml_str = serde_yaml::to_string(&self).unwrap();

        match fs::write(&config_file, &yaml_str) {
            Ok(_) => { },
            Err(e) => {
                error!("Unable to write to configuratoni file, {}, error: {}", config_file, e.to_string());
                std::process::exit(0);
            }
        };
    }

}

impl Default for CiceroServerConfig {
    fn default() -> CiceroServerConfig {
        CiceroServerConfig {
            general: ConfigGeneral::default(),
            daemons: ConfigDaemons::default(),
            ml: ConfigML::default(),
            items_per_batch: ConfigItemsPerBatch::default(),
            models: ConfigModels::default(),
            api_keys: HashMap::new()
        }
    }
}

impl Default for ConfigGeneral {
    fn default() -> ConfigGeneral {
        ConfigGeneral {
            mode: "normal".to_string(),
            libdir: String::new(),
            plugin_dev_dir: String::new(),
            language: "en".to_string(),
            agree_share: false,
            num_threads: 0,
            use_gpu: false,
            network_mode: "local".to_string(),
            api_client: String::new(),
            ollama_url: String::new()
        }
    }
}

impl Default for ConfigDaemons {
    fn default() -> ConfigDaemons {
        ConfigDaemons {
            apollo: ("local".to_string(), 7511),
            helios: ("0.0.0.0".to_string(), 6834),
            echo: ("127.0.0.1".to_string(), 5833),
        }
    }
}

impl Default for ConfigML {
    fn default() -> ConfigML {
        ConfigML {
            embedding_chunk_maxlength: 1000,
            embedding_search_threshold: 0.8
        }
    }
}

impl Default for ConfigItemsPerBatch {
    fn default() -> ConfigItemsPerBatch {
        ConfigItemsPerBatch {
            summarization: 5,
            sentence_embeddings: 5
        }
    }
}

impl Default for ConfigModels {
    fn default() -> ConfigModels {
        ConfigModels {
            autoload: String::new(),
            summarization: String::from("text-summarizer-bart-large-cnn-samsum"),
            text_generation: String::from("t5-base"),
            question_answer: String::from("bert-large-cased-whole-word-masking-finetuned-squad"),
            ner: String::from("bert-ner'"),
            keyword_extraction: String::from("all-roberta-large-v1"),
            zero_shot_classification: String::from("facebook/bart-large-mnli"),
            pos_tagging: String::from("mobilebert-finetuned-pos'"),
            sentiment_analysis: String::from("distilbert-base-uncased-finetuned-sst-2-english"),
            sentence_embeddings: String::from("all-roberta-large-v1'"),
            sequence_classification: String::from("distilbert-base-uncased-finetuned-sst-2-english',"),
            token_classification: String::from("bert-ner")
        }
    }
}




