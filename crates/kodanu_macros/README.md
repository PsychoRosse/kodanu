# kodanu_macros

Utility macros for the [Kodanu](https://github.com/sneakypin/kodanu) game engine.

`kodanu_macros` provides the [`Getters`](https://docs.rs/kodanu_macros/latest/kodanu_macros/derive.Getters.html) derive macro and [`map_with_capacity!`](https://docs.rs/kodanu_macros/latest/kodanu_macros/macro.map_with_capacity.html), a macro that creates a `HashMap` with a pre-allocated capacity and inserts the given key-value pairs.

## Features

* `Getters` derive that generates read-only accessors for struct fields
* Configurable getters: `copy`, `str`, `must_use`, and custom method names
* Pre-sized `HashMap` construction from literal key-value pairs
* Comma-separated pairs with an optional trailing comma
* Prelude with commonly used macros

## Usage

Add `kodanu_macros` to your dependencies:

```toml
[dependencies]
kodanu_macros = "0.1.0"
```

### Getters

Derive read-only accessors for the fields of a struct:

```rust
use kodanu_macros::Getters;

#[derive(Getters)]
struct WindowAttributes {
    #[getter(str)]
    title: &'static str,
    #[getter(copy, must_use)]
    maximized: bool,
    #[getter(name = "raw_id")]
    id: usize,
}

let attributes = WindowAttributes {
    title: "Kodanu",
    maximized: false,
    id: 0,
};

assert_eq!(attributes.title(), "Kodanu");
assert!(!attributes.maximized());
assert_eq!(attributes.raw_id(), &0);
```

Without options the generated method is a `const fn` that returns a reference to the field. The `#[getter(...)]` field attribute accepts:

| Option | Effect |
| ------ | ------ |
| `copy` | Return the field by value instead of by reference; the field type must be `Copy` |
| `str` | Return the field as `&str`; cannot be combined with `copy` |
| `must_use` | Mark the method with `#[must_use]` |
| `name = "method"` | Name the method `method` instead of the field name |

Doc comments written on a field are copied to the generated method. Tuple structs are supported as well; their fields are accessed through `field_0`, `field_1`, and so on.

### map_with_capacity

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
