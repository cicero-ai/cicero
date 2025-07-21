
use faiss::{Index, FlatIndex, IdMap, Idx, MetricType, ConcurrentIndex, index_factory};
use faiss::index::IndexImpl;
use faiss::index::io;
use cicero::preludes::*;
use x25519_dalek::PublicKey;
use std::path::Path;
use std::fs;
use cicero::security::UserKey;
use cicero::utils::sys;
use crate::cfx::CfxStorage;
use super::embeddings;
use crate::Error;
use log::error;

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
    ) -> Result<Self, Error> {

        // Create directory, if needed
        if !Path::new(&datadir).exists() {
            fs::create_dir_all(&datadir)
                .map_err(|e| Error::IO(e.to_string()) )?;
        }

        let rocksdb_dir = format!("{}/kvstore", datadir.trim_end_matches("/"));
        let mut findex = index_factory(64, "HNSW", MetricType::L2)
            .map_err(|e| Error::Faiss(e.to_string()) )?;

        let mut faiss = Self {
            datadir: datadir.trim_end_matches("/").to_string(),
            id_counter: 0,
            index: IdMap::new(findex).unwrap(),
            storage: CfxStorage::new(&rocksdb_dir, &user_key, &public_key)?
        };

        // Get id number
        faiss.get_id_counter();
        Ok(faiss)
    }

    /// Get id counter
    fn get_id_counter(&mut self) {
        let counter_file = format!("{}/id_counter", self.datadir);
        if Path::new(&counter_file).exists() {
            self.id_counter = fs::read_to_string(&counter_file)
                .unwrap_or("0".to_string())
                .trim()
                .parse()
                .unwrap_or(0);
        }
    }

    fn save_id_counter(&self) -> Result<(), Error> {
        let counter_file = format!("{}/id_counter", self.datadir);
        fs::write(&counter_file, self.id_counter.to_string())
            .map_err(|e| Error::IO(format!("Failed to write id_counter: {}", e)))
    }
    /// Add to index
    pub fn add(&mut self, embeddings: &[f32], original_text: &str) -> Result<(), Error> {

        self.id_counter += 1;
        let idx = Idx::new(self.id_counter);

        self.index.add_with_ids(&embeddings, &[idx])
            .map_err(|e| Error::Faiss( format!("Unable to add to faiss index, datadir: {}, error: {}", self.datadir, e)) )?;

        self.storage.put(&format!("{}", self.id_counter).as_str(), original_text.as_bytes());
        Ok(())
    }

    /// Save index
    pub fn save(&mut self) -> Result<(), Error> {
        let index_file = format!("{}/index.faiss", self.datadir);
        io::write_index(&self.index, &index_file).unwrap();
        self.save_id_counter()
    }

    /// Load
    pub fn load(&mut self) -> Result<(), Error> {
        let index_file = format!("{}/index.faiss", self.datadir);
        let findex = io::read_index(&index_file)
            .map_err(|e| Error::Faiss(e.to_string()) )?; 

        self.index = IdMap::new(findex)
            .map_err(|e| Error::Faiss(e.to_string()) )?;

        Ok(())
    }

    /// Load if exists
    pub fn load_if_exists(&mut self) -> Result<(), Error> {
        let index_file = format!("{}/index.faiss", self.datadir);
        if !Path::new(&index_file).exists() {
            return Ok(());
        }
        self.load()
    }

    /// Search by string
    pub fn search_str(&mut self, query: &str, max_results: usize) -> Result<Vec<String>, Error> {

        // Get embeddings
        let embeddings = embeddings::generate(&vec![query.to_string()])?;
        let search_result = self.index.search(&embeddings[0].as_slice(), max_results).unwrap();

        // Get results
        let mut res = Vec::new();
        for lbl in search_result.labels {

            let key = format!("{}", lbl.get().unwrap());
            if let Some(value) = self.storage.get(&key)? {
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
            .expect("Unable to create default faiss index!")
    }
}

