# string-error

This crate provides a simple way to use a string as an error trait object,
i.e. `Box<std::error::Error>`.

If you need more sophisticated error handling, you should consider
[error-chain](https://crates.io/crates/error-chain), which also provides
functionality to create simple errors from Strings.

## Compatibility

This crate works with Stable Rust (tested with 1.20.0) and has no
dependencies.

## License

Written by Ulrich Rhein, licensed under the Apache License 2.0.

See [COPYRIGHT](COPYRIGHT) and [LICENSE](LICENSE) for details.

## Usage

In your `Cargo.toml`:
```
[dependencies]
string-error = "0.1.0"
```

In your code:
```rust
extern crate string_error;

use std::error::Error;
use string_error::{into_err, new_err, static_err};

static ERROR_MESSAGE : &'static str = "This is a constant error message";

fn use_static_err() -> Result<(), Box<Error>> {
    // creates an error from a static str
    Err(static_err(ERROR_MESSAGE))
}

fn use_new_err() -> Result<(), Box<Error>> {
    let x = String::from("Create an error from an owned string.");
    Err(new_err(&x)) // copies x
}

fn use_into_err() -> Result<(), Box<Error>> {
    let x = String::from("Create an error from an owned string.");
    Err(into_err(x)) // takes ownership of x
}
```