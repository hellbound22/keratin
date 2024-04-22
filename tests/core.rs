use rand::prelude::Rng;
use serde::{Deserialize, Serialize};

use keratin::*;

#[derive(Clone, Serialize, Deserialize)]
struct TestStruct{
    n: String
}

#[test]
fn failed_insert() {
    let se = keratin::storage::LocalFsStorage;
    let mut coll: Collection<TestStruct> = Collection::configure(None, &se).unwrap();

    coll.truncate();

    // Until a truncate option is made, the second will panic in the first run and the first will
    // panic in the second run
    // once truncate is made, only the second one should fail
    assert!(coll.insert("key", TestStruct{n: "teste".to_owned()}).is_ok());
    assert!(coll.insert("key", TestStruct{n: "teste".to_owned()}).is_err());
}

#[test]
fn test_fast_setup() {
    let se = keratin::storage::LocalFsStorage;
    let _coll: Collection<TestStruct> = Collection::configure(None, &se).unwrap();
}

#[test]
fn modify() {
    let se = keratin::storage::LocalFsStorage;
    let mut coll: Collection<TestStruct> = Collection::configure(None, &se).unwrap();

    coll.truncate();

/*
    match coll.delete("modifytest") {
        Ok(_) => {},
        Err(_) => {}
    }
*/    

    coll.insert("modifytest", TestStruct{n: "ass".to_owned()}).unwrap();
    assert_eq!(coll.get("modifytest").unwrap().n, "ass");

    coll.modify("modifytest", TestStruct{n: "boobs".to_owned()}).unwrap();
    assert_eq!(coll.get("modifytest").unwrap().n, "boobs");
}

#[test]
fn random_insert_and_delete() {
    let se = keratin::storage::LocalFsStorage;
    let mut coll: Collection<TestStruct> = Collection::configure(None, &se).unwrap();

    coll.truncate();

    let key = "random_key";

    let mut rng = rand::thread_rng();
    let nmr = rng.gen_range(0, 100).to_string();
    let result = coll.insert(key, TestStruct{n: nmr.clone()});

    assert!(result.is_ok());
    assert_eq!(coll.get(key).unwrap().n, nmr);
    assert!(coll.delete(key).is_ok());
}
