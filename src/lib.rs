use bson::Document;

use std::collections::HashMap;
use std::path::Path;

mod config;

use config::*;

/// Represents a collection of documents.
///
/// It is the main API for data managment for Keratin.
#[derive(Clone, Debug)]
pub struct Collection {
    main_path: String,
    config: Config,
    mapped_keys: Option<HashMap<String, String>>, // Pair (key, path to document)
    cached_docs: Option<HashMap<String, Document>>, // Pair (key, document)
}

impl Collection {
    pub fn insert(entry: &str) {}

    pub fn delete(query: &str) {}

    pub fn modify(query: &str) {}
    /// A function to create a new Keratin db from scratch for a fast setup.
    ///
    /// This truncates every document in ```db/data/```.
    /// The config file keratin.toml is created with the default options. If it already exists, the
    /// config file AND the mapped keys file (map.bson) will be left alone.
    ///
    /// # Panics
    ///
    /// This fuction uses the enviroment variable ```CARGO_MANIFEST_DIR```, so this will only work
    /// when running your project using ```cargo```, else it will panic.
    /// If you're using planning in using Keratin in production use ```configure()``` instead
    pub fn new(n: &str) -> Collection {
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

        Collection {
            main_path,
            config,
            mapped_keys: None,
            cached_docs: None,
        }
    }
}
