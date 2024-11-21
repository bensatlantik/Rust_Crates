## Archived Repository

**This repository was a gift from me to the Rust and open-source community. It is no longer actively maintained and has been archived. Feel free to fork and continue development on your own.**

# riscv_ai_infer 🚀
A Rust-based, lightweight AI inference engine optimized for **RISC-V boards**. This project aims to enable efficient AI model inference on RISC-V systems, especially useful for resource-constrained environments like IoT devices and edge computing.

## 📜 Table of Contents
- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Example Output](#example-output)
- [Contributing](#contributing)
- [License](#license)

## 📝 Introduction
`riscv_ai_infer` is a tool designed to perform AI model inference on **RISC-V hardware**. Leveraging Rust’s performance and safety guarantees, this engine is optimized for low-power, resource-constrained devices, making it ideal for edge computing and IoT applications.

## ✨ Features
- Efficient AI inference using Rust and `nalgebra`.
- Optimized for RISC-V architecture.
- Caching system to reduce redundant API calls.
- Terminal-based user interface for quick insights.
- Flexible configuration options.

## ⚙️ Installation
First, ensure you have the Rust toolchain installed. Then, clone this repository and build the project:

```bash
git clone https://github.com/bensatlantik/riscv_ai_infer.git
cd riscv_ai_infer
cargo build --release
```
## Usage
You can run the program directly using:
```bash
cargo run
```
## Command-Line Arguments (Optional)
To specify a crate for which you want to fetch statistics:
```bash
cargo run -- <crate_name>
```
## Configuration
The tool supports a configuration file named config.toml. This is optional and will be skipped if not present.
```toml
# Example config.toml
api_key = ""

```
## Example Output
```yaml
No configuration file found. Skipping...
Cache data: Object {"example_key": String("example_value")}
Crate: serde
Total Downloads: 0
No recent downloads data available.
```
## License
This project is licensed under the MIT License

## Author
Ben Santora <bensatlantik@gmail.com>
