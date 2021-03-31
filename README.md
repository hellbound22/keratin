# Keratin
### A quick setup/quick development, embedded, modular database

Keratin is designed from the start to be simple but expansive. 
Works with any serializable and deserializable struct.

In the future you'll be able to choose how to structure Keratin: how you'll interact with it, what format of persistant data you'll use, attach your own custom modules for data treatment...

WIP!!! Probably don't use in production

### Exemple
```rust
use keratin::*;

fn main() {
    // Create the collection (using None as the parameter defaults to a directory inside the project)
    let mut coll: Collection<String> = Collection::configure(None);

    // Generate your data
    let mut rng = rand::thread_rng();
    let nmr = rng.gen_range(0, 100).to_string();

    // Insert the data into the collection
    let result = coll.insert("random_key", nmr.clone());
    assert!(result.is_ok());
    
    // Get the data from the collection
    let retrieved_data = coll.get("random_key").unwrap().inner();
    assert_eq!(retrieved_data, &nmr);
    
    // Delete the entry
    coll.delete(&key);
    
    // Modify the data entry
    coll.modify("random_key", "modifying this entry".to_string()).unwrap();
    
    let retrieved_data = coll.get("random_key").unwrap().inner();
    assert_eq!(retrieved_data, "modifying this entry");
}

```

#### TODO:
- [ ] Modularize the library

#### Default project directory Layout
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

