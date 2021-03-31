use rand::prelude::Rng;
use keratin::*;

const PATH: &str  = "db/keratin.toml";

#[test]
#[should_panic]
fn failed_insert() {
    let mut coll: Collection<String> = Collection::configure(Some(PATH));

    coll.insert("key", "teste".to_string()).unwrap();
}

#[test]
fn test_fast_setup() {
    let _coll: Collection<String> = Collection::configure(None);
}

#[test]
fn modify() {
    let mut coll: Collection<String> = Collection::configure(Some(PATH));

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
    let mut coll: Collection<String> = Collection::configure(Some(PATH));

    let key = "random_key";

    let mut rng = rand::thread_rng();
    let nmr = rng.gen_range(0, 100).to_string();
    let result = coll.insert(key, nmr.clone());

    assert!(result.is_ok());
    assert_eq!(coll.get(key).unwrap().inner(), &nmr);
    assert!(coll.delete(&key).is_ok());
}
