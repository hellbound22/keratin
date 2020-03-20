# Keratin
### A quick setup/quick development, embedded, modular database

Keratin is designed from the start to be simple but expansive. 

### Exemple
```rust
use keratin::*;

fn main() {
	// Create the collection
	let mut db = Collection::new();

	// Every insert auto persists the data. No need to "confirm changes" or manage the state
	db.insert(r#"{"data": "not so important data here"}"#);

	// Both query and delete use regex
	db.delete("regex string here");

	for doc in db.query("regex string here") {
		// inner() returns a reference to the value inside the result of query()
		dbg!(doc.inner());


		db.delete_by_key(doc.key());
	}
}

```

#### Directory Layout
```
project folder ---src/
				|
				|-Cargo.toml
				|
				|-target/
				|
				|-db/---------keratin.toml (config)
							|
							|-map.bson (mapped keys)
							|
							|-data/ ------- BSON documents
```

