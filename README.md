# this-error-from-box (vibe-coded)

This crate provides a procedural macro for Rust that automatically generates `From<T>` implementations for error enums annotated with `#[this_error_from_box]` and variants containing `#[from] Box<T>`.

You can optionally specify a custom wrapper type as an argument, e.g. `#[this_error_from_box(MyWrapper)]`. If omitted, `Box` is used by default.

## Example

```rust
use thiserror::Error;
use this_error_from_box::this_error_from_box;

#[derive(Error, Debug)]
#[this_error_from_box]
pub enum MyError {
    Io(#[from] Box<std::io::Error>),
    Utf8(#[from] Box<std::string::FromUtf8Error>),
    // other variants ...
}
```

This will automatically generate the following implementations:

```rust
impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> Self {
        MyError::Io(Box::from(e))
    }
}

impl From<std::string::FromUtf8Error> for MyError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        MyError::Utf8(Box::from(e))
    }
}
```

## Installation

```bash
cargo add this-error-from-box
```

## Notes
- The macro only works for variants with exactly one field of type `Box<T>` and the attribute `#[from]`.
- No `From<T>` implementation is generated for other variants.
