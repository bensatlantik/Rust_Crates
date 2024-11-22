## Sinewave Generator

A Rust application that generates a 440 Hz sine wave and saves it as a `.wav` file.

## Overview

This project creates a simple sine wave audio file (`sine_wave_440hz.wav`) using the `hound` crate for handling the WAV format. The generated audio is 2 seconds long, with a single channel (mono) and a sample rate of 44.1 kHz.

## How to Run

Clone this repository or navigate to the `sinewave_gen` directory.

Build the project:
```bash
   cargo build
```
Run the project:
```bash
   cargo run
```
This will generate the sine_wave_440hz.wav file in the current directory.

## Dependencies
hound: A Rust library for reading and writing WAV audio files

## License
This project is open source and available under the MIT License

## Author
Ben Santora <bensatlantik@gmail.com>
