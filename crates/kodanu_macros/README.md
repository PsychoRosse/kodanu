# kodanu_macros

Utility macros for the [Kodanu](https://github.com/sneakypin/kodanu) game engine.

`kodanu_macros` provides [`map_with_capacity!`](https://docs.rs/kodanu_macros/latest/kodanu_macros/macro.map_with_capacity.html), a macro that creates a `HashMap` with a pre-allocated capacity and inserts the given key-value pairs.

## Features

* Pre-sized `HashMap` construction from literal key-value pairs
* Comma-separated pairs with an optional trailing comma
* Prelude with commonly used macros

## Usage

```rust
use {kodanu_macros::map_with_capacity, std::collections::HashMap};

let map: HashMap<&str, i32> = map_with_capacity!(2; "a" => 1, "b" => 2);
```

The first argument reserves capacity, the rest is a list of `key => value` pairs:

```rust
let map: HashMap<&str, i32> = map_with_capacity!(4; "name" => "kodanu");
```

The macro expands to `HashMap::with_capacity`, so `std::collections::HashMap` must be in scope at the call site.

Alternatively, import through the prelude:

```rust
use kodanu_macros::prelude::*;
```

## License

Licensed under either:

* MIT License
* Apache License, Version 2.0

at your option.
