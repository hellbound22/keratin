use bson::{decode_document, encode_document, Bson, Document};

use std::collections::HashMap;
use std::fs::{self, DirBuilder, DirEntry, File};
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub mod config;
pub mod errors;

use config::*;
use errors::*;

/// Represents a collection of documents.
///
/// It is the main API for data managment for Keratin.
#[derive(Clone, Debug)]
pub struct Collection {
    main_path: String,
    config: Config,
    //mapped_keys: HashMap<String, String>, // Pair (key, path to document)
    cached_docs: HashMap<String, String>, // Pair (key, entry)
}

impl Collection {
    fn _gen_key(&self, pk: &str) -> String {
        let digest = md5::compute(pk);

        format!("{}{:x}", self.config.coll_prefix(), digest)
    }

    pub fn get(&mut self, k: &str) -> Option<String> {
        let key = self._gen_key(k);
        self._find(&key)
    }

    fn _find(&mut self, pk: &str) -> Option<String> {
        self.cache_entries();
        match self.cached_docs.get(pk) {
            Some(x) => {
                let entry = x.clone();
                let doc = decode_document(&mut entry.as_bytes()).expect("Could Not Decode");

                return Some(doc.get("data").unwrap().as_str().unwrap().to_string());
            }
            None => return None,
        }
    }

    pub fn insert(&mut self, entry: &str) -> Result<(), Errors> {
        // Generate primary key
        let key = self._gen_key(entry);

        // Check if entry with key already exists in cache
        match self._find(&key) {
            Some(_) => Err(Errors::AlreadyExists),
            None => {
                // Write the entry to a document and save it
                let mut doc = Document::new();
                doc.insert("data".to_owned(), Bson::String(entry.to_owned()));

                let mut buf = Vec::new();
                encode_document(&mut buf, &doc).unwrap();

                let mut file =
                    File::create(format!("{}/{}.bson", self.config.data_path(), key)).unwrap();
                file.write_all(&buf).unwrap();

                Ok(())
            }
        }
    }

    pub fn delete(query: &str) {}

    pub fn modify(query: &str) {}
    /// A function to create a new Keratin db from scratch for a fast setup.
    ///
    /// The config file keratin.toml is created with the default options. If it already exists, the
    /// config file.
    ///
    /// # Arguments
    /// Truncate: if it is TRUE, this function wipes every document in ```db/data/``` along with
    /// truncating the mapped keys file.
    ///
    /// # Panics
    ///
    /// This fuction uses the enviroment variable ```CARGO_MANIFEST_DIR```, so this will only work
    /// when running your project using ```cargo```, else it will panic.
    /// If you're using planning in using Keratin in production use ```configure()``` instead
    pub fn new(truncate: bool) -> Collection {
        unimplemented!()
    }

    /// A function to initialize the collection using the path of a configuration file
    ///
    /// # Arguments
    ///
    /// * `path` - An Option with a Path. If this is None, Keratin will use the default config file
    /// path (eg. ```db/keratin.toml```)
    ///
    /// # Attention
    ///
    /// USE ONLY ABSOLUTE PATHS!!!
    ///
    /// # Errors
    ///
    /// This returns an error if the config file is not found OR if the folder doesn't have the
    /// right permitions
    // TODO: Error handle this
    pub fn configure(path: &str) -> Collection {
        let path = Path::new(path);

        let config = Config::new_from_path(path);
        let main_path = String::from(path.parent().unwrap().to_str().unwrap());

        DirBuilder::new()
            .recursive(true)
            .create(config.data_path())
            .unwrap();

        Collection {
            main_path,
            config,
            //mapped_keys: HashMap::new(),
            cached_docs: HashMap::new(),
        }
    }

    // TODO: Error handle this
    pub fn cache_entries(&mut self) {
        for entry in fs::read_dir(self.config.data_path()).unwrap() {
            let fp = entry.unwrap().path();
            let mut f = File::open(fp.clone()).unwrap();
            let mut s = String::new();
            f.read_to_string(&mut s).unwrap();

            let key = Path::new(&fp).file_stem();

            self.cached_docs
                .insert(key.unwrap().to_str().unwrap().to_string(), s);
        }
    }
}
