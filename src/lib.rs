use std::collections::HashMap;
use std::fs::{DirBuilder, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use serde::ser::Serialize;
use serde::de::Deserialize;
use serde;

pub mod config;
pub mod errors;
pub mod storage;

use config::*;
use errors::*;
use storage::*;

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
pub struct Entry<T> {
    pub key: String,
    pub content: T,
}
impl<T> Entry<T> {
    //pub fn as_bytes(&self) -> &[u8] {
    //    self.content.as_bytes()
    //}

    pub fn inner(&self) -> &T {
        &self.content
    }

    pub fn _inner_mut(&mut self) -> &mut T {
        &mut self.content
    }
}

/// Represents a collection of documents.
///
/// It is the main API for data managment for Keratin.
pub struct Collection<'a, T> {
    //main_path: String,
    config: Config,
    cached_docs: HashMap<String, Entry<T>>, // Pair (key, entry)
    storage_engine: &'a (dyn StorageEngine<T>)

}

impl<'a, T: Serialize + for<'de> Deserialize<'de>> Collection<'a, T> {
    fn _gen_key(&self, pk: &str) -> String {
        let digest = md5::compute(pk);

        format!("{}{:x}", self.config.coll_prefix(), digest)
    }

    /// Returns an entry of the database given the respective key, or ```None``` if the key
    /// corresponds to no known entries
    pub fn get(&mut self, k: &str) -> Option<&Entry<T>> {
        let key = self._gen_key(k);
        self._find(&key)
    }

    fn _find(&mut self, pk: &str) -> Option<&Entry<T>> {
        self.cached_docs = self.storage_engine.cache_entries(self.config.data_path());
        self.cached_docs.get(pk)
    }

    /// Insert an entry into the database given an ```Entry```
    ///
    /// # Note
    /// This does not cache the entry automaticaly 
    pub fn insert(&mut self, key: &str, entry: T) -> Result<(), Errors> {
        // Generate primary key
        let k = self._gen_key(key);

        // Check if entry with key already exists in cache
        match self._find(&k) {
            Some(_) => Err(Errors::AlreadyExists),
            None => {
                // Write the entry to a document and save it
                self.storage_engine.write_record(self.config.data_path(), entry, &k);
                self.cached_docs = self.storage_engine.cache_entries(self.config.data_path());
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
        
        let ret = self.storage_engine.remove_entry(self.config.data_path(), &k);
        self.cached_docs = self.storage_engine.cache_entries(self.config.data_path());
        //self.cached_docs.remove(&k).unwrap();
        return ret
    }

    pub fn modify(&mut self, key: &str, new_entry: T) -> Result<(), Errors> {
        let k = self._gen_key(key);

        match self._find(&k) {
            None => Err(Errors::EntryNotFound),
            Some(_) => {
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
    pub fn new(self, truncate: bool) -> Result<Collection<'a, T>, Errors> {
        // TODO: Actually remove all files on db/data
        if truncate {
            let path = generate_default_config_structure();

            let config = Config::new_from_path(&path);

            DirBuilder::new()
                .recursive(true)
                .create(config.data_path())
                .unwrap();

            return Ok(Collection {
                config,
                cached_docs: HashMap::new(),
                storage_engine: &LocalFsStorage
            })
        } else {
            let path = Path::new("db/keratin.toml");
            let config = Config::new_from_path(&path);
       
            return Ok(Collection {
                config,
                cached_docs: HashMap::new(),
                storage_engine: &LocalFsStorage
            })
            
        }
        //Err(Errors::DbConfigurationError)
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
    pub fn configure(path: Option<&str>, se: &'a (dyn StorageEngine<T>)) -> Collection<'a, T> {
        let path = match path {
            Some(x) => PathBuf::from(x),
            None => {
                generate_default_config_structure()
            }
        };

        let config = Config::new_from_path(&path);

        DirBuilder::new()
            .recursive(true)
            .create(config.data_path())
            .unwrap();

        Collection {
            config,
            cached_docs: HashMap::new(),
            storage_engine: se
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
