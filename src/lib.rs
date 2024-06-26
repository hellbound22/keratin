use std::collections::HashMap;
use std::fs::{DirBuilder, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use serde::ser::Serialize;
use serde::de::Deserialize;
use serde;

use anyhow::Result;

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


/// Represents a collection of documents.
///
/// It is the main API for data managment for Keratin.
#[derive(Clone)]
pub struct Collection<'a, T> {
    //main_path: String,
    config: Config,
    cached_docs: HashMap<String, T>, // Pair (key, entry)
    storage_engine: &'a (dyn StorageEngine<T>)

}

impl<'a, T: Serialize + Clone + for<'de> Deserialize<'de>> Collection<'a, T> {
    fn _gen_key(&self, pk: &str) -> String {
        let digest = md5::compute(pk);

        format!("{}{:x}", self.config.coll_prefix(), digest)
    }

    /// Returns an entry of the database given the respective key, or ```None``` if the key
    /// corresponds to no known entries
    pub fn get(&mut self, k: &str) -> Option<T> {
        let key = self._gen_key(k);
        let ret = self._find(&key);

        if let Some(e) = ret.clone() {
            self.cached_docs.insert(key, e);
        }

        ret
    }

    fn _find(&mut self, pk: &str) -> Option<T> {
        // TODO: first try to find in cache, if not found, fallback to engine get()
        // self.cached_docs = self.storage_engine.cache_entries(self.config.data_path(), &self.config.coll_prefix());
        if let Some(e) = self.cached_docs.remove(pk) {
            Some(e)
        } else {
            match self.storage_engine.find_in_storage(self.config.data_path(), pk) {
                Some(e) => {
                        Some(e)
                    },
                None => {
                    None
                }
            }
        }
    }

    pub fn truncate(&mut self) -> Result<()> {
        self.storage_engine.truncate_all(self.config.data_path())
    }

    /// Insert an entry into the database given an ```Entry```
    ///
    /// # Note
    /// This does not cache the entry automaticaly 
    pub fn insert(&mut self, key: &str, entry: T) -> Result<()> {
        // Generate primary key
        let k = self._gen_key(key);

        // Check if entry with key already exists 
        match self._find(&k) {
            Some(_) => Err(Errors::AlreadyExists.into()),
            None => {
                // Write the entry to a document and save it
                self.storage_engine.write_record(self.config.data_path(), entry, &k)
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
    pub fn delete(&mut self, query: &str) -> Result<()> {
        let k = self._gen_key(query);
        
        let ret = self.storage_engine.remove_entry(self.config.data_path(), &k)?;
        self.cached_docs.remove(&k);
        Ok(ret)
    }

    pub fn modify(&mut self, key: &str, new_entry: T) -> Result<()> {
        let k = self._gen_key(key);

        match self._find(&k) {
            None => Err(Errors::EntryNotFound.into()),
            Some(_) => {
                self.delete(key)?;
                self.insert(key, new_entry)?;
                
                Ok(())
            }
        }

    }

    /// A function to initialize the collection using the path of a configuration file
    ///
    /// # Arguments
    ///
    /// * `path` - An Option with a Path. If this is None, Keratin will use the default config file
    /// path (eg. ```db/keratin.toml```)
    ///
    /// * `se` - The `Storage Engine` of the database. Right now only `LocalFsStorage` is
    /// implemented into the crate, but in theory anything that implements the `StorageEngine`
    /// trait could be passed as the parameter. 
    ///
    /// # Errors
    ///
    /// This returns an error if the config file is not found OR if the folder doesn't have the
    /// right permitions
    // TODO: Error handle this
    pub fn configure(path: Option<&str>, se: &'a (dyn StorageEngine<T>)) -> Result<Collection<'a, T>> {
        let path = match path {
            Some(x) => PathBuf::from(x),
            None => {
                generate_default_config_structure()?
            }
        };

        let config = Config::new_from_path(&path)?;

        DirBuilder::new()
            .recursive(true)
            .create(config.data_path())?;

        Ok(Collection {
            config,
            cached_docs: HashMap::new(),
            storage_engine: se
        })
    }

    pub fn iter_mut(&mut self) -> Result<std::collections::hash_map::IterMut<String, T>> {
        self.cached_docs = self.storage_engine.cache_entries(self.config.data_path(), &self.config.coll_prefix())?;
        Ok(self.cached_docs.iter_mut())
    }
}

fn generate_default_config_structure() -> Result<PathBuf> {
    DirBuilder::new()
        .recursive(true)
        .create("db")?; 
    let mut f = File::create("db/keratin.toml")?;

    f.write_all(DEFAULT_CONFIG.as_bytes())?;
    
    Ok(Path::new("db/keratin.toml").to_owned())
}
