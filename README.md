# this-error-from-box (vibe-coded)

This crate provides a procedural macro for Rust that automatically generates `From<T>` implementations for error enums annotated with `#[this_error_from_box]` and variants containing `#[from] Box<T>`.

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
        MyError::Io(Box::new(e))
    }
}

impl From<std::string::FromUtf8Error> for MyError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        MyError::Utf8(Box::new(e))
    }
}
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
thiserror = "*"
this-error-from-box = { git = "https://github.com/Relacibo/this-error-from-box.git" }

```

## Notes
- The macro only works for variants with exactly one field of type `Box<T>` and the attribute `#[from]`.
- No `From<T>` implementation is generated for other variants.
