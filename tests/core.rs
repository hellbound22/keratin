use rand::prelude::Rng;
use keratin::*;

const PATH: &str  = "db/keratin.toml";

#[test]
#[should_panic]
fn failed_insert() {
    let se = keratin::storage::LocalFsStorage;
    let mut coll: Collection<String> = Collection::configure(None, &se);

    // Until a truncate option is made, the second will panic in the first run and the first will
    // panic in the second run
    // once truncate is made, only the second one should fail
    coll.insert("key", "teste".to_string()).unwrap();
    coll.insert("key", "testeagain".to_string()).unwrap();
}

#[test]
fn test_fast_setup() {
    let se = keratin::storage::LocalFsStorage;
    let _coll: Collection<String> = Collection::configure(None, &se);
}

#[test]
fn modify() {
    let se = keratin::storage::LocalFsStorage;
    let mut coll: Collection<String> = Collection::configure(None, &se);

    match coll.delete("modifytest") {
        Ok(_) => {},
        Err(_) => {}
    }
    

    coll.insert("modifytest", "ass".to_string()).unwrap();
    assert_eq!(coll.get("modifytest").unwrap().inner(), "ass");

    coll.modify("modifytest", "boobs".to_string()).unwrap();
    assert_eq!(coll.get("modifytest").unwrap().inner(), "boobs");
}

#[test]
fn random_insert_and_delete() {
    let se = keratin::storage::LocalFsStorage;
    let mut coll: Collection<String> = Collection::configure(None, &se);

    let key = "random_key";

    let mut rng = rand::thread_rng();
    let nmr = rng.gen_range(0, 100).to_string();
    let result = coll.insert(key, nmr.clone());

    assert!(result.is_ok());
    assert_eq!(coll.get(key).unwrap().inner(), &nmr);
    assert!(coll.delete(key).is_ok());
}
