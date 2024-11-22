# PWM Signal Generator
Created by bensatlantik

## Overview
`pwm_gen` is a simple Rust application that generates a Pulse Width Modulation (PWM) signal. This program demonstrates the basic concept of PWM by simulating PWM signals via console output. This version is for educational purposes.

## Extending the PWM Generator for Embedded Systems
To make your PWM generator control GPIO pins, you'll typically use a library specific to the hardware platform:

- **Raspberry Pi**: Using the `rppal` crate for GPIO control.
- **STM32 (ARM Cortex-M)**: Using the `stm32f4xx-hal` crate.
- **General Embedded Rust**: The `embedded-hal` crate provides a hardware abstraction layer for embedded devices.

## Installation
Clone the repository and navigate to the project directory:
```sh
git clone https://github.com/yourusername/pwm_gen
cd pwm_gen
```
Build and run the application using Cargo:
```rust
cargo build --release
./target/release/pwm_gen
```
## Customization
You can adjust the duty cycle and frequency by modifying the parameters in src/main.rs:
```
fn main() {
    let pwm = Pwm::new(0.5, 1); // 50% duty cycle, 1 Hz frequency
    pwm.start();
}
```
## License
This project is licensed under the MIT License

## Author
bensatlantik
