use anyhow::Result;
use rand::prelude::Rng;

use keratin::*;

#[test]
fn failed_insert() -> Result<()> {
    let se = keratin::storage::LocalFsStorage;
    let mut coll: Collection<String> = Collection::configure(None, &se)?;

    coll.truncate()?;

    // Until a truncate option is made, the second will panic in the first run and the first will
    // panic in the second run
    // once truncate is made, only the second one should fail
    assert!(coll.insert("key", "teste".to_owned()).is_ok());
    assert!(coll.insert("key", "teste".to_owned()).is_err());

    Ok(())
}

#[test]
fn test_fast_setup() -> Result<()> {
    let se = keratin::storage::LocalFsStorage;
    let _coll: Collection<String> = Collection::configure(None, &se)?;

    Ok(())
}

#[test]
fn modify() -> Result<()> {
    let se = keratin::storage::LocalFsStorage;
    let mut coll: Collection<String> = Collection::configure(None, &se)?;

    coll.truncate()?;

    coll.insert("modifytest", "ass".to_owned())?;
    assert_eq!(coll.get("modifytest").unwrap(), "ass");

    coll.modify("modifytest", "boobs".to_owned())?;
    assert_eq!(coll.get("modifytest").unwrap(), "boobs");

    Ok(())
}

#[test]
fn random_insert_and_delete() -> Result<()> {
    let se = keratin::storage::LocalFsStorage;
    let mut coll: Collection<String> = Collection::configure(None, &se)?;

    coll.truncate()?;

    let key = "random_key";

    let mut rng = rand::thread_rng();
    let nmr = rng.gen_range(0, 100).to_string();
    let result = coll.insert(key, nmr.clone());

    assert!(result.is_ok());
    assert_eq!(coll.get(key).unwrap(), nmr);
    assert!(coll.delete(key).is_ok());

    Ok(())
}
