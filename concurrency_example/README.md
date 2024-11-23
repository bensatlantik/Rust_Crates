## concurrency_example

## Overview
This is a simple Rust program demonstrating concurrency using threads. The program spawns a new thread that runs in parallel with the main thread. Both threads print messages to the terminal, but with different sleep intervals, allowing you to observe how they execute concurrently.

## How It Works
The spawned thread prints a message every 500 milliseconds.
The main thread prints a message every 1000 milliseconds.
Once the main thread finishes, it waits for the spawned thread to complete using join().

## Running the Program
Make sure you have Rust installed on your system. If not, download it from rust-lang.org.

## Clone the repository and run the program using:
```bash
cargo run
```
Expected Output:
You should see alternating messages from the main thread and the spawned thread in your terminal:
```bash
Hi from the main thread: 1
Hi from the spawned thread: 1
Hi from the spawned thread: 2
Hi from the main thread: 2
...
```
## Notes
This program runs in the terminal, so make sure to run it from a terminal emulator.
The output might appear differently depending on your terminal's behavior with concurrent prints.

## License
This project is licensed under the MIT License

## Author
bensatlantik