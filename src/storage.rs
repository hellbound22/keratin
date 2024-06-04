use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{Path};

use bson::{Document};
use bson::{to_document, from_document};

use anyhow::Result;

use serde::{Deserialize, Serialize};

use crate::errors::*;

pub trait StorageEngine<T> {
    fn cache_entries(&self, data_path: &str, coll_prefix: &str) -> Result<HashMap<String, T>>;

    fn truncate_all(&self, data_path: &str) -> Result<()>;

    fn remove_entry(&self, data_path: &str, given_key: &str) -> Result<()>;

    fn write_record(&self, data_path: &str, entry: T, key: &str) -> Result<()>;

    fn find_in_storage(&self, data_path: &str, key: &str) -> Option<T>;
}

#[derive(Clone, Debug)]
pub struct LocalFsStorage;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Record<T> {
    metadata: Option<bool>, // TODO: Implement metadata struct
    data: T
}

impl<T: Serialize + for<'de> Deserialize<'de>> StorageEngine<T> for LocalFsStorage 
    //where Document: From<T> 
    {

    fn find_in_storage(&self, data_path: &str, key: &str) -> Option<T> {
        match File::open(format!("{}/{}.bson", data_path, key)) {
            Ok(mut f) => {
                let inter = Document::from_reader(&mut f).expect("Could Not Decode");

                let e: Record<T> = match from_document(inter) {
                    Ok(r) => {r},
                    Err(_e) => { return None }, // WARN: this should problaly return an error if it fails
                                        // to parse the document as T
                };
                Some(e.data)
            },
            Err(_) => { None }
        }
    }

    fn truncate_all(&self, data_path: &str) -> Result<()> {
        let walker = match fs::read_dir(data_path) {
            Ok(w) => w,
            Err(_) => return Err(Errors::FsError.into())
        };
        for entry in walker {
            if let Err(_e) = fs::remove_file(entry?.path()) {
                return Err(Errors::FsError.into());
            };
        }
        Ok(())
    }

    fn cache_entries(&self, data_path: &str, _coll_prefix: &str) -> Result<HashMap<String, T>> {
        let mut hm = HashMap::new();

        let walker = match fs::read_dir(data_path) {
            Ok(w) => w,
            Err(_) => return Err(Errors::FsError.into())
        };
        for entry in walker {
            let fp = entry?.path();
            let mut f = File::open(fp.clone())?;

            let key = Path::new(&fp)
                .file_stem().ok_or::<anyhow::Error>(Errors::FsError.into())?
                .to_str().ok_or::<anyhow::Error>(Errors::FsError.into())?.to_string();

            let doc = Document::from_reader(&mut f).expect("Could Not Decode");
            
            let upd: Record<T> = from_document(doc)?;

            hm.insert(key, upd.data);
        }
    
        return Ok(hm)
    }

    fn remove_entry(&self, data_path: &str, given_key: &str) -> Result<()> {
        for entry in fs::read_dir(data_path)? {
            let fp = entry?.path();

            let key = Path::new(&fp)
                .file_stem().ok_or::<anyhow::Error>(Errors::FsError.into())?
                .to_str().ok_or::<anyhow::Error>(Errors::FsError.into())?.to_string();

            if key == given_key {
                fs::remove_file(fp)?;

                return Ok(())
            }
        }
        Err(Errors::EntryNotFound.into())
    }

    fn write_record(&self, data_path: &str, entry: T, key: &str) -> Result<()> {
        let rec = Record {
            metadata: None,
            data: entry
        };
        let doc = to_document(&rec)?;

        let mut s = Vec::new();
        doc.to_writer(&mut s)?;

        let mut file =
            File::create(format!("{}/{}.bson", data_path, key))?;
        file.write_all(&s)?;
        Ok(())
    }
}
