## Archived Repository

**This repository was a gift from me to the Rust and open-source community. It is no longer actively maintained and has been archived. Feel free to fork and continue development on your own.**

## FFT Analyzer

## Overview
This Rust program reads a .wav audio file, performs a Fast Fourier Transform (FFT) on the audio data, and generates a .png plot of the frequency spectrum. The included audio file, waves.wav, is a recording of ocean waves, which is rich in diverse frequencies. The resulting FFT plot helps visualize the frequency content of the audio.

## Features
Reads .wav files and extracts audio samples.
Performs FFT to analyze the frequency spectrum.
Plots the magnitude of frequencies in decibels (dB) using the plotters crate.
Saves the plot as an image (fft_output.png).

##Prerequisites
To run this project, you'll need:
Rust (latest stable version). If you don't have Rust installed, you can download it from rust-lang.org.
The following Rust crates:
hound for reading .wav files.
rustfft for performing the FFT.
plotters for generating plots.

## Installation & Setup
Clone the repository and navigate into the project directory:
```bash
git clone <repository-url>
cd fft-analyzer
```
Install the required dependencies:
```
cargo build
```
Once the dependencies are installed, run the program using:
```
cargo run
```
The program will:
 - Read the included waves.wav file
 - Perform an FFT on the audio samples
 - Generate a plot of the frequency spectrum in fft_output.png
 
## Customizing the Input
If you wish to analyze a different .wav file, replace the existing waves.wav file in the project directory with your own. Ensure that the new file has the same name (waves.wav) or modify the source code to use a different file name.

## Dependencies
The Cargo.toml file includes the following dependencies:
```toml
[dependencies]
hound = "3.4"
rustfft = "6.1"
plotters = "0.3.1"
```
## License
This project is licensed under the MIT License

## Acknowledgments
Rust community and documentation
Crate maintainers for hound, rustfft, and plotters

## Author
Ben Santora <bensatlantik@gmail.com>


