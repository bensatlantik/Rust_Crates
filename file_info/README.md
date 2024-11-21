## Archived Repository

**This repository was a gift from me to the Rust and open-source community. It is no longer actively maintained and has been archived. Feel free to fork and continue development on your own.**

## file_info

## Overview
This program demonstrates how to inspect file metadata using the Rust standard library. It reads the metadata of a specified file and prints details such as the file size and type.

## How It Works
The program uses the fs::metadata function to get metadata information about the specified file.
It checks if the given path is a regular file, a directory, or another type.
It prints the file size (in bytes) and its type to the terminal.

## Running the Program
Ensure Rust is installed on your system. You can download it from rust-lang.org.

## Setup
Clone the repository:
```bash
git clone <repository-url>
cd file_info
```
Create a file named example.txt in the project directory or modify the filename in the code to match an existing file.
Build and run the program:
```bash
cargo run
```
## Expected Output
If the file example.txt exists, you'll see output similar to:
```
File Size: 123 bytes
File Type: Regular File
```

## Notes
This program is designed to be run in the terminal.
It currently works on Unix-based systems due to its use of MetadataExt from std::os::unix.

## License
This project is licensed under the MIT License

## Author
Ben Santora <bensatlantik@gmail.com>