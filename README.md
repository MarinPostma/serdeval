# SerdeVal
A serde validator

[crates.io](https://crates.io/crates/serdeval)

## Overview

SerdeVal allows you to validate data that can be deserialized using serde, whithout actually deserializing to anything. This is usefull when you only want to validate that some data can be deserialized to some type. SerdeVal doesn't allocate anything, so it is extremely efficient for validating large files from disk:

```rust
use std::io::File;

use serdeval::*;

// we want to check that the very_big.json is an arrray of javascript objects:
fn main() {
	let json = File::open("very_big.json").unwrap();
	let _: Seq<Map<Str, Any>> = serde_json::from_reader(json).unwarp();
}
```

## License: MIT
