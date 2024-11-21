## Archived Repository
This repository was a gift from me to the Rust and open-source community. It is no longer actively maintained and has been archived. Feel free to fork and continue development on your own.

## Equation Plotter
A Rust program to plot mathematical equations and save them as PNG files.

## Features
- **Plot Mathematical Equations**: Generate and save plots of mathematical equations.
- **Flexible and Extendable**: Easily modify the code to plot different equations.
- **High-Quality PNG Output**: Save the plots as high-resolution PNG images.

## Prerequisites

Before running the `equation_plotter`, ensure you have the following installed:

### On Debian/Ubuntu:
```sh
sudo apt update
sudo apt install pkg-config libfontconfig1-dev
```

## Installation

## Clone the repository:
```sh
git clone https://github.com/bensatlantik/Rust_Binaries.git
cd Rust_Binaries/equation_plotter
```

## Build the Project
```sh
cargo build --release
```

## Run the Program
```sh
cargo Run
```

## Usage
You can modify the main.rs file to plot different equations by changing the closure passed to the plot_equation function.

Example:
To plot the equation y = 3x + 5, update main.rs as follows:
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let linear = |x: f64| 3.0 * x + 5.0;
    plot_equation(linear, "linear.png")
}
```

## License
This project is licensed under the MIT License.

## Author
Ben Santora <bensatlantik@gmail.com>
