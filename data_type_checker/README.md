## Archived Repository

**This repository was a gift from me to the Rust and open-source community. It is no longer actively maintained and has been archived. Feel free to fork and continue development on your own.**

# TypeChecker

`data_type_checker` is a lightweight Rust library for validating and converting string data into different data types, making it easy to handle dynamic inputs in your applications.

## Installation

Add `type_checker` to your `Cargo.toml`:

```toml
[dependencies]
data_type_checker = "0.1.0"```

## Usage
```rust
use data_type_checker::TypeChecker;

fn main() {
    let num_str = "42";

    if TypeChecker::is_int(num_str) {
        println!("'{}' is an integer!", num_str);
    }

    if let Some(value) = TypeChecker::to_int(num_str) {
        println!("Parsed integer: {}", value);
    }
}
```
## Features
is_int: Checks if a string can be parsed as an integer.
is_float: Checks if a string can be parsed as a float.
is_bool: Checks if a string represents a boolean.
to_int: Attempts to parse a string into an integer, returning None if it fails.
to_float: Attempts to parse a string into a float, returning None if it fails.
to_bool: Attempts to parse a string into a boolean, returning None if it fails.

## License
This project is licensed under the MIT License

## Author
Ben Santora 
