use bson::{decode_document, encode_document, Bson, Document};

use std::collections::HashMap;
use std::fs::{self, DirBuilder, File};
use std::io::prelude::*;
use std::io::Cursor;
use std::path::{Path, PathBuf};

pub mod config;
pub mod errors;

use config::*;
use errors::*;

const DEFAULT_CONFIG: &'static str = r#"project = ".default."
                    [core]
                    collection = ".default."
                    primary_key = "id"
                    data_path = ".default."
                    "#;

/// Represents an Entry in the database.
/// 
/// Contains the key and the following content corresponding to that key.
#[derive(Clone, Debug)]
pub struct Entry {
    pub key: String,
    pub content: String,
}
impl Entry {
    pub fn as_bytes(&self) -> &[u8] {
        self.content.as_bytes()
    }

    pub fn inner(&self) -> &str {
        &self.content
    }

    pub fn _inner_mut(&mut self) -> &mut str {
        &mut self.content
    }
}

/// Represents a collection of documents.
///
/// It is the main API for data managment for Keratin.
#[derive(Clone, Debug)]
pub struct Collection {
    main_path: String,
    config: Config,
    //mapped_keys: HashMap<String, String>, // Pair (key, path to document)
    cached_docs: HashMap<String, Entry>, // Pair (key, entry)
}

impl Collection {
    fn _gen_key(&self, pk: &str) -> String {
        let digest = md5::compute(pk);

        format!("{}{:x}", self.config.coll_prefix(), digest)
    }

    /// Returns an entry of the database given the respective key, or ```None``` if the key
    /// corresponds to no known entries
    pub fn get(&mut self, k: &str) -> Option<&Entry> {
        let key = self._gen_key(k);
        self._find(&key)
    }

    fn _find(&mut self, pk: &str) -> Option<&Entry> {
        self.cache_entries();
        self.cached_docs.get(pk)
    }

    fn _write_record(&self, entry: &str, key: &str) {
        let mut doc = Document::new();
        doc.insert("data".to_owned(), Bson::String(entry.to_owned()));

        let mut buf = Vec::new();
        encode_document(&mut buf, &doc).unwrap();

        let mut file =
            File::create(format!("{}/{}.bson", self.config.data_path(), key)).unwrap();
        file.write_all(&buf).unwrap();
    }

    /// Insert an entry into the database given an ```Entry```
    ///
    /// # Note
    /// This does not cache the entry automaticaly 
    pub fn insert(&mut self, key: &str, entry: &str) -> Result<(), Errors> {
        // Generate primary key
        let k = self._gen_key(key);

        // Check if entry with key already exists in cache
        match self._find(&k) {
            Some(_) => Err(Errors::AlreadyExists),
            None => {
                // Write the entry to a document and save it
                self._write_record(entry, &k);
                Ok(())
            }
        }
    }

    /// Delete a entry in the database given the key.
    /// 
    /// This deletes from both the cache and non-volatile storage.
    /// 
    /// # Note
    /// In the future this will use a query string to find what multiple elements to delete
    ///
    /// # Return
    /// Returns an Error ```EntryNotFound``` if the key does not match any entry
    pub fn delete(&mut self, query: &str) -> Result<(), Errors>{
        let k = self._gen_key(query);
        return self._remove_entry(&k)
    }

    pub fn modify(&mut self, key: &str, new_entry: &str) -> Result<(), Errors> {
        let k = self._gen_key(key);

        match self._find(&k) {
            None => Err(Errors::EntryNotFound),
            Some(x) => {
                self.delete(key).unwrap();
                self.insert(key, new_entry).unwrap();
                
                Ok(())
            }
        }

    }
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
    pub fn new(self, truncate: bool) -> Result<Collection, Errors> {
        // TODO: Actually remove all files on db/data
        if truncate {
            let path = generate_default_config_structure();

            let config = Config::new_from_path(&path);
            let main_path = String::from(path.parent().unwrap().to_str().unwrap());

            DirBuilder::new()
                .recursive(true)
                .create(config.data_path())
                .unwrap();

            return Ok(Collection {
                main_path,
                config,
                //mapped_keys: HashMap::new(),
                cached_docs: HashMap::new(),
            })
        } else {
            let path = Path::new("db/keratin.toml");
            let config = Config::new_from_path(&path);
            let main_path = String::from(path.parent().unwrap().to_str().unwrap());

       
            return Ok(Collection {
                main_path,
                config,
                //mapped_keys: HashMap::new(),
                cached_docs: HashMap::new(),
            })
            
        }
        Err(Errors::DbConfigurationError)
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
    /// Use of the ```None``` Option is unstable
    ///
    /// # Errors
    ///
    /// This returns an error if the config file is not found OR if the folder doesn't have the
    /// right permitions
    // TODO: Error handle this
    pub fn configure(path: Option<&str>) -> Collection {
        let path = match path {
            Some(x) => PathBuf::from(x),
            None => {
                generate_default_config_structure()
            }
        };

        let config = Config::new_from_path(&path);
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
    fn _remove_entry(&mut self, given_key: &str) -> Result<(), Errors>{
        for entry in fs::read_dir(self.config.data_path()).unwrap() {
            let fp = entry.unwrap().path();
            let key = Path::new(&fp).file_stem().unwrap().to_str().unwrap().to_string();

            if key == given_key {
                fs::remove_file(fp).unwrap();
                self.cached_docs.remove(given_key);

                return Ok(())
            }
        }
        Err(Errors::EntryNotFound)
    }

    // TODO: Error handle this
    pub fn cache_entries(&mut self) {
        for entry in fs::read_dir(self.config.data_path()).unwrap() {
            let fp = entry.unwrap().path();
            //let mut f = File::open(fp.clone()).unwrap();
            let s = String::from_utf8_lossy(&fs::read(fp.clone()).unwrap()).into_owned();

            let key = Path::new(&fp).file_stem().unwrap().to_str().unwrap().to_string();

            let doc = decode_document(&mut Cursor::new(s)).expect("Could Not Decode");
            let upd = doc.get("data").unwrap().as_str().unwrap().to_string();

            let e = Entry {
                key: key.clone(),
                content: upd.clone()
            };


            self.cached_docs.insert(key, e);
        }
    }
}


fn generate_default_config_structure() -> PathBuf {
    DirBuilder::new()
        .recursive(true)
        .create("db")
        .unwrap();
    let mut f = File::create("db/keratin.toml").unwrap();

    f.write_all(DEFAULT_CONFIG.as_bytes()).unwrap();
    
    Path::new("db/keratin.toml").to_owned()
}
