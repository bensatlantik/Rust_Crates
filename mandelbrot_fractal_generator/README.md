## Archived Repository
This repository was a gift from me to the Rust and open-source community. It is no longer actively maintained and has been archived. Feel free to fork and continue development on your own.

## Getting Started
These instructions will help you set up and run the Mandelbrot fractal generator on your local machine.

## Prerequisites
Rust and Cargo: You need to have Rust and Cargo installed.

## Installation
Clone the repository
```sh
git clone https://github.com/bensatlantik/mandelbrot-fractal-generator.git
cd mandelbrot-fractal-generator
```

Build the Project
```sh
cargo build --release
```

## Dependencies
This project depends on the image crate for image creation and manipulation. Make sure it is included in your Cargo.toml:
```toml
[dependencies]
image = "0.23.14"
```

## Usage
Run the project to generate the Mandelbrot fractal image:
```
cargo run --release
```
The program will create an image file named mandelbrot_fractal.png in the project directory.

## Customization
You can customize the fractal generation by modifying the constants in the source code:

WIDTH and HEIGHT: Change the dimensions of the output image.

MAX_ITER: Adjust the maximum number of iterations to influence the fractal detail.

X_MIN, X_MAX, Y_MIN, and Y_MAX: Modify the boundaries of the complex plane being rendered.

## Output
The output is an 800x800 image of the Mandelbrot fractal, saved as mandelbrot_fractal.png. The color gradient represents the iteration count, providing a visual representation of the fractal's complexity.

## License
This project is licensed under the MIT License

## Author
Ben Santora <bensatlantik@gmail.com>
