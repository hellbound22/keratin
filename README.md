# Keratin
### A quick setup/quick development, embedded, modular database

Keratin is designed from the start to be simple but expansive. 
Works with any serializable and deserializable struct.

In the future you'll be able to choose how to structure Keratin: how you'll interact with it, what format of persistant data you'll use, attach your own custom modules for data treatment...

WIP!!! Probably don't use in production

Run tests with ```cargo test -- --test-threads 1```

### Exemple
```rust
use keratin::*;

fn main() -> Result<()> {
	// Choose your method of storage (Anything that implements the 'StorageEngine' trait should work)
    let se = keratin::storage::LocalFsStorage;

    // Create the collection (using None as the parameter defaults to a directory inside the project)
    let mut coll: Collection<String> = Collection::configure(None, &se)?;


    // Insert the data into the collection
    let result = coll.insert("key", "something".to_owned())?;
    assert!(result.is_ok());
    
    // Get the data from the collection
    let retrieved_data = coll.get(key).unwrap();
    assert_eq!(retrieved_data, "something");
    
    // Delete the entry
    coll.delete(&key)?;
    
    // Modify the data entry
    coll.modify("key", "another something".to_owned())?;
    
    let retrieved_data = coll.get("key").unwrap();
    assert_eq!(retrieved_data, "another something");
}

```

#### Avalible engines

- Storage
	- LocalFsStorage
		- Stores records in the local file system as BSON files

#### TODO:
- [ ] Metadata field
    - Partially implemented
- [ ] Implement the engines
	- [x] StorageEngine
	- [ ] CacheEngine (caches records and helps the queries)
        - [x] partial Implement
	- [ ] QueryEngine (query the records)
        - Missing trait
	- [ ] NetworkEngine (access other instances of keratin via the nerwork)
	- [ ] ConfigurationEngine (configure keratin some other way?)
	- [ ] AuthenticationEngine (Access and control keratin instances via credentials)

#### Default project directory Layout using the defaut configuration
```
project folder ---src/
				|
				|-Cargo.toml
				|
				|-target/
				|
				|-db/---------keratin.toml (config)
							|
							|-data/ ------- BSON documents
```

