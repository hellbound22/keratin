use keratin::errors::Errors;
use keratin::*;

#[test]
#[should_panic]
fn failed_insert() {
    let path = "/home/rodrigo/Software/Rust/keratin/db/keratin.toml";

    let mut coll = Collection::configure(path);

    coll.insert("teste").unwrap();
}

#[test]
fn get() {
    let path = "/home/rodrigo/Software/Rust/keratin/db/keratin.toml";

    let mut coll = Collection::configure(path);

    assert!(coll.get("teste").is_some())
}
