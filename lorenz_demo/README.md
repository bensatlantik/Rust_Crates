## Lorenz System Demonstrator

## Overview
`lorenz_demo` is a Rust project that demonstrates the principles of chaos theory using the Lorenz system. The project simulates the Lorenz system and visualizes the results, showcasing how small changes in initial conditions can lead to vastly different outcomes.

## Features
- Simulates the Lorenz system using a simple Euler method
- Visualizes the Lorenz attractor using the `plotters` crate
- Provides an educational demonstration of chaos theory

## Installation
Clone the repository and navigate to the project directory:
```sh
git clone https://github.com/yourusername/lorenz_demo
cd lorenz_demo
```
Build and run the project to generate the Lorenz attractor plot:
```shell
cargo build --release
./target/release/lorenz_demo
```
The plot will be saved as lorenz.png in the project directory.

## License
This project is licensed under the MIT License

## Author
bensatlantik
