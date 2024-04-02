use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{Path};

use bson::{Document};
use bson::{to_document, from_document};

use serde::ser::Serialize;
use serde::de::Deserialize;

use crate::errors::*;

pub trait StorageEngine<T> {
    fn cache_entries(&self, data_path: &str, coll_prefix: &str) -> HashMap<String, T>;

    fn truncate_all(&self, data_path: &str);

    fn remove_entry(&self, data_path: &str, given_key: &str) -> Result<(), Errors>;

    fn write_record(&self, data_path: &str, entry: T, key: &str);

    fn find_in_storage(&self, data_path: &str, key: &str) -> Option<T>;
}

#[derive(Clone, Debug)]
pub struct LocalFsStorage;

impl<T: Serialize + for<'de> Deserialize<'de>> StorageEngine<T> for LocalFsStorage 
    where Document: From<T> 
    {

    fn find_in_storage(&self, data_path: &str, key: &str) -> Option<T> {
        match File::open(format!("{}/{}.bson", data_path, key)) {
            Ok(mut f) => {
                let inter = Document::from_reader(&mut f).expect("Could Not Decode");

                let e = from_document(inter).unwrap();
                Some(e)
            },
            Err(_) => { None }
        }
    }

    fn truncate_all(&self, data_path: &str) {
        for entry in fs::read_dir(data_path).unwrap() {
            fs::remove_file(entry.unwrap().path()).unwrap();
        }
    }

    fn cache_entries(&self, data_path: &str, coll_prefix: &str) -> HashMap<String, T> {
        let mut hm = HashMap::new();
        for entry in fs::read_dir(data_path).unwrap() {
            let fp = entry.unwrap().path();
            let mut f = File::open(fp.clone()).unwrap();

            let key = Path::new(&fp).file_stem().unwrap().to_str().unwrap().to_string();

            let doc = Document::from_reader(&mut f).expect("Could Not Decode");
            
            let upd: T = from_document(doc).unwrap();


           hm.insert(key, upd);
        }
    
        return hm
    }

    fn remove_entry(&self, data_path: &str, given_key: &str) -> Result<(), Errors> {
        for entry in fs::read_dir(data_path).unwrap() {
            let fp = entry.unwrap().path();
            let key = Path::new(&fp).file_stem().unwrap().to_str().unwrap().to_string();

            if key == given_key {
                fs::remove_file(fp).unwrap();
                //cache.remove(given_key);

                return Ok(())
            }
        }
        Err(Errors::EntryNotFound)
    }

    fn write_record(&self, data_path: &str, entry: T, key: &str) {
        let doc = to_document(&entry).unwrap();

        let mut s = Vec::new();
        doc.to_writer(&mut s).unwrap();

        let mut file =
            File::create(format!("{}/{}.bson", data_path, key)).unwrap();
        file.write_all(&s).unwrap();
    }
}
