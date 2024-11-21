## Archived Repository

**This repository was a gift from me to the Rust and open-source community. It is no longer actively maintained and has been archived. Feel free to fork and continue development on your own.**

## abi_lab

abi_lab is an experimental Rust library and binary designed to facilitate advanced exploration and understanding of Application Binary Interface (ABI) concepts, particularly as they relate to cross-language interoperability and Rust’s Foreign Function Interface (FFI).

## Key Features
Data Structure Layouts: Examples of #[repr(C)] data structures to understand how Rust data can be made ABI-compliant.
Cross-Language Function Calls: extern "C" functions that show how data is passed and modified between Rust and other languages.
Function Callbacks: Examples of passing function pointers between Rust and C for event-driven programming.
Error Handling: Demonstrations of handling errors gracefully when interfacing with C.
Memory Management: Examples covering safe memory management practices across the FFI boundary.
Interactive Binary: A binary (main.rs) showcasing practical examples of all key features for easy experimentation.

# Use Cases
abi_lab is suitable for:

Developers interested in systems programming and understanding how Rust can interface with C and other languages.
Experiments and educational purposes, offering a foundation to explore the implications of ABI stability (or lack thereof) in Rust.
Those looking to learn advanced FFI techniques like callbacks and memory safety practices.

## Example Code
Simple Data Example:
```rust
use abi_lab::{SimpleData, modify_simple_data};

fn main() {
    let mut data = SimpleData {
        value: 0,
        flag: false,
    };

    modify_simple_data(&mut data as *mut _);
    println!("Value: {}, Flag: {}", data.value, data.flag);
}
```
Callback Example:
```rust
use abi_lab::{CallbackData, register_callback, trigger_callback};

extern "C" fn callback_fn(val: i32) {
    println!("Callback called with value: {}", val);
}

fn main() {
    let mut callback_data = CallbackData {
        value: 42,
        callback: None,
    };

    register_callback(&mut callback_data as *mut _, callback_fn);
    trigger_callback(&mut callback_data as *mut _);
}
```
ComplexData Example:
```rust
use abi_lab::{ComplexData, process_complex_data};

extern "C" fn complex_callback(data: *const ComplexData) {
    unsafe {
        println!("ComplexData Callback with ID: {}", (*data).id);
    }
}

fn main() {
    let mut complex_data = ComplexData {
        id: 1,
        name: "Example".as_ptr(),
        name_len: 7,
        values: [1.0, 2.0, 3.0, 4.0, 5.0],
        callback: Some(complex_callback),
    };

    process_complex_data(&mut complex_data as *mut _);
}
```
## Running the Binary
To explore abi_lab interactively, run the binary:
```sh
cargo run
```
Expected Output:
```plaintext
Before modification: Value = 10, Flag = false
After modification: Value = 11, Flag = true

Registering and triggering a callback...
Callback triggered with value: 42

Processing ComplexData...
ComplexData Callback triggered with ID: 99, Name: Example
```
## License
Distributed under the MIT license

## Author
Ben Santora <bensatlantik@gmail.com>



