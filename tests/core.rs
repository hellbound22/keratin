use keratin::*;

#[test]
fn show_collection() {
    let path = "/home/rodrigo/Software/Rust/keratin/db/keratin.toml";

    let mut coll = Collection::configure(path);

    dbg!(&coll);

    assert!(coll.exists("teste"));
}
