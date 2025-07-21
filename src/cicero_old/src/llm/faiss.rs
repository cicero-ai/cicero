
use faiss::{Index, FlatIndex, IdMap, Idx, MetricType, ConcurrentIndex, index_factory};
use faiss::index::IndexImpl;
use faiss::index::io;
use x25519_dalek::PublicKey;
use crate::server::security::UserKey;
use crate::error::Error;
use crate::server::cfx::CfxStorage;
use crate::llm::nlp;
use crate::utils::sys;
use log::error;
use std::path::Path;
use std::fs;

pub struct FaissIndex {
    datadir: String,
    id_counter: u64,
    pub index: IdMap<IndexImpl>,
    storage: CfxStorage
}

impl FaissIndex {

    pub fn new(
        datadir: &str,
        user_key: &UserKey,
        public_key: &PublicKey
    ) -> Self {

        // Create directory, if needed
        if !Path::new(&datadir).exists() {
            match fs::create_dir_all(&datadir) {
                Ok(_) => {}
                Err(_error) => panic!("Unable to create directory at {}.", datadir),
            };
        }
        let rocksdb_dir = format!("{}/kvstore", datadir.trim_end_matches("/"));
        //let mut findex = index_factory(64, "Flat", MetricType::L2).unwrap();
        let mut findex = index_factory(64, "HNSW", MetricType::L2).unwrap();

        let mut faiss = Self {
            datadir: datadir.trim_end_matches("/").to_string(),
            id_counter: 0,
            index: IdMap::new(findex).unwrap(),
            storage: CfxStorage::new(&rocksdb_dir.as_str(), &user_key, &public_key)
        };

        // Get id number
        faiss.get_id_counter();
        faiss
    }

    /// Get id counter
    fn get_id_counter(&mut self) {

        let counter_file = format!("{}/id_counter", self.datadir);
        if !Path::new(&counter_file).exists() {
            self.save_id_counter();
        }

        self.id_counter = fs::read_to_string(counter_file).unwrap().trim().parse::<u64>().unwrap();
    }

    /// Save id counter
    fn save_id_counter(&mut self) {

        let counter_file = format!("{}/id_counter", self.datadir);
        match fs::write(counter_file.clone(), format!("{}", self.id_counter)) {
            Ok(_) => { },
            Err(e) => {
                error!("Unable to write to id_counter file at {}.  Error: {}", counter_file, e);
                std::process::exit(1);
            }
        };

    }

    /// Add to index
    pub fn add(&mut self, embeddings: &[f32], original_text: &str) -> Result<(), Error> {

        self.id_counter += 1;
        let idx = Idx::new(self.id_counter);

        match self.index.add_with_ids(&embeddings, &[idx]) {
            Ok(_) => { },
            Err(e) => {
                error!("Unable to add to FAISS index {}, error: {}", self.datadir, e);
                std::process::exit(1);
            }
        };
        self.storage.put(&format!("{}", self.id_counter).as_str(), original_text.as_bytes());

        Ok(())
    }

    /// Save index
    pub fn save(&mut self) {
        let index_file = format!("{}/index.faiss", self.datadir);
        io::write_index(&self.index, &index_file).unwrap();
        self.save_id_counter();
    }

    /// Load
    pub fn load(&mut self) -> Result<(), Error> {
        let index_file = format!("{}/index.faiss", self.datadir);
        let findex = match io::read_index(&index_file) {
            Ok(r) => r,
            Err(e) => {
                error!("Unable to load FAISS index at {}, error: {}", index_file, e);
                std::process::exit(1);
            }
        };
        self.index = IdMap::new(findex).unwrap();
        Ok(())
    }

    /// Load if exists
    pub fn load_if_exists(&mut self) -> Result<(), Error> {
        let index_file = format!("{}/index.faiss", self.datadir);
        if !Path::new(&index_file).exists() {
            return Ok(());
        }
        self.load()?;
        Ok(())
    }

    /// Search by string
    pub fn search_str(&mut self, query: &str, max_results: usize) -> Result<Vec<String>, Error> {

        // Get embeddings
        let embeddings = nlp::sentence_embeddings(&vec![query.to_string()])?;
        let search_result = self.index.search(&embeddings[0].as_slice(), max_results).unwrap();

        // Get results
        let mut res = Vec::new();
        for lbl in search_result.labels {

            let key = format!("{}", lbl.get().unwrap());
            if let Some(value) = self.storage.get(&key) {
                res.push(String::from_utf8(value).unwrap());
            }

            if res.len() >= max_results {
                break;
            }
        }

        Ok(res)
    }

}

impl Default for FaissIndex {
    fn default() -> FaissIndex {
        let user_key = UserKey::generate();
        let pub_key = UserKey::generate();

        let datadir = format!("{}/profiles/guest/faiss/profile", sys::get_datadir());
        FaissIndex::new(&datadir.as_str(), &user_key, &pub_key.public)
    }
}

