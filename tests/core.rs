use rand::prelude::*;

use keratin::errors::Errors;
use keratin::*;

const PATH: &str  = "/home/rodrigo/Software/Rust/keratin/db/keratin.toml";

#[test]
#[should_panic]
fn failed_insert() {
    let mut coll = Collection::configure(Some(PATH));

    coll.insert("key", "teste").unwrap();
}

#[test]
fn test_fast_setup() {
    let mut coll = Collection::configure(None);

    assert!(coll.get("key").is_some())
}

#[test]
fn get() {
    let mut coll = Collection::configure(Some(PATH));

    assert!(coll.get("key").is_some())
}

#[test]
fn random_insert_and_delete() {
    let mut coll = Collection::configure(Some(PATH));

    let key = "randon_key";

    let mut rng = rand::thread_rng();
    let nmr = rng.gen_range(0, 100).to_string();

    assert!(coll.insert(key, &nmr).is_ok());
    assert_eq!(&coll.get(key).unwrap().inner().to_string(), &nmr);
    assert!(coll.delete(&key).is_ok());
}
