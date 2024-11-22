# one_liner_stats

## Overview
`one_liner_stats` is a Rust library that computes descriptive statistics (mean, median, mode, etc.) on the fly for numeric slices or streams.

## Features
- Calculate mean, median, and mode for a given dataset.
- Simple and efficient implementation.

## Usage

## Add Dependency
Add `one_liner_stats` to your `Cargo.toml`:
```toml
[dependencies]
one_liner_stats = "0.1.0"
```
## Example
```rust
use one_liner_stats::{mean, median, mode};

fn main() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    let mean_value = mean(&data);
    let mut data_for_median = data.clone();
    let median_value = median(&mut data_for_median);
    let mode_value = mode(&data);

    println!("Mean: {}", mean_value);
    println!("Median: {}", median_value);
    println!("Mode: {:?}", mode_value);
}
```
something wrong - toggles between high and low in my terminal at high rate locking it up
It looks like the high frequency is causing the terminal to become unresponsive due to the rapid toggling. Let's add a delay to make it more manageable and visible.

Adjusted PWM Code with Adjustable Delay
Update src/main.rs: Modify your code to include a delay to slow down the toggling, making it more readable:

rust
use std::thread;
use std::time::Duration;

struct Pwm {
    duty_cycle: f32, // Duty cycle as a percentage (0.0 to 1.0)
    frequency: u32,  // Frequency in Hz
}

impl Pwm {
    fn new(duty_cycle: f32, frequency: u32) -> Self {
        Pwm { duty_cycle, frequency }
    }

    fn start(&self) {
        let period = Duration::from_micros((1_000_000 / self.frequency) as u64);
        let high_time = period.mul_f32(self.duty_cycle);
        let low_time = period - high_time;

        loop {
            // Simulate high signal
            println!("HIGH");
            thread::sleep(high_time);

            // Simulate low signal
            println!("LOW");
            thread::sleep(low_time);

            // Add delay to make the output more readable
            thread::sleep(Duration::from_millis(100)); // 100 ms delay
        }
    }
}

fn main() {
    let pwm = Pwm::new(0.5, 1); // 50% duty cycle, 1 Hz frequency
    pwm.start();
}
Build and Run:

sh
cargo build --release
./target/release/pwm_gen
By adding thread::sleep(Duration::from_millis(100));, the toggling rate will slow down to a more readable speed, preventing the terminal from locking up.

Let me know if this works better! 😊✨

that fixed it - i now need .toml and a good README
Great! Let’s create a Cargo.toml and a solid README for your PWM generator.

Cargo.toml
Here’s your Cargo.toml file with the necessary metadata:

toml
[package]
name = "pwm_gen"
version = "0.1.0"
authors = ["Your Name <your.email@example.com>"]
edition = "2021"

# Description and repository (optional)
description = "A simple PWM signal generator in Rust"
# Uncomment the next line if you wish to include a repository link
# repository = "https://github.com/yourusername/pwm_gen"

[dependencies]
README.md
And here’s a comprehensive README for your project:

markdown
# PWM Signal Generator

## Overview

`pwm_gen` is a simple Rust application that generates a Pulse Width Modulation (PWM) signal. This program demonstrates the basic concept of PWM by simulating the signal in the console. It's a great starting point for understanding how PWM works and can be extended for actual hardware use.

## Features

- Configurable duty cycle and frequency
- Simple and easy-to-understand code
- Suitable for educational purposes and basic PWM simulations

## Usage

### Installation

Clone the repository and navigate to the project directory:

```sh
git clone https://github.com/yourusername/pwm_gen
cd pwm_gen
Running the Application
You can build and run the application using Cargo:

sh
cargo build --release
./target/release/pwm_gen
Customization
You can adjust the duty cycle and frequency by modifying the parameters in src/main.rs:

rust
fn main() {
    let pwm = Pwm::new(0.5, 1); // 50% duty cycle, 1 Hz frequency
    pwm.start();
}
Contributing
Contributions are welcome! If you have ideas for improvements or new features, feel free to open an issue or submit a pull request.

License
This project is licensed under the MIT License. See the LICENSE file for details.

Contact
For any questions or support, feel free to reach out to Your Name at your.email@example.com.

Happy coding! 😊✨


This README provides an overview of the project, installation instructions, usage examples, and contribution guidelines. Feel free to customize it further based on your preferences.

Does this look good to you? If you need any more adjustments or additions, let me know! 🚀📜✨
create mode 100644 src/main.rs ben@HP-17:~/pwm_gen$ git push -u origin main error: src refspec main does not match any error: failed to push some refs to 'https://github.com/bensatlantik/pwm_gen.git' ben@HP-17:~/pwm_gen$ - this happens when i use gh repo create / cargo init --bin - it still needs to be intializied with url?
The error message you're seeing occurs because the main branch does not exist in the remote repository. By default, newer versions of GitHub use the main branch, but sometimes this needs to be explicitly created or set up.

Steps to Resolve the Issue:
Initialize the Git Repository and Add Remote: If you haven't already, initialize your Git repository and add the remote URL:

sh
git init
git remote add origin https://github.com/bensatlantik/pwm_gen.git
Check Your Branch: Ensure that you are on the correct branch. If you just initialized the repository, you might be on the master branch. You can check your current branch with:

sh
git branch
Create and Switch to the main Branch: If you're not already on the main branch, create and switch to it:

sh
git checkout -b main
Add and Commit Your Changes: Add and commit your files:

sh
git add .
git commit -m "Initial commit"
Push to the main Branch: Now push your changes to the main branch on GitHub:

sh
git push -u origin main
By following these steps, you should be able to push your code to the main branch on GitHub successfully.

Let me know if this resolves the issue! 😊🚀✨

got it - can i set my github / account / repo to always choose main and never master?
Yes, you can configure Git to use main as the default branch for new repositories. Here's how you can set this up:

Setting Default Branch to main on GitHub
Sign In to GitHub: Log in to your GitHub account.

Navigate to Your Settings: Go to your profile picture in the top-right corner, click on it, and then select Settings.

Repositories: In the left sidebar, click Repositories.

Default Branch: Scroll down to the Repository default branch section and set the default branch name to main. This will apply to all new repositories you create.

Setting Default Branch Locally for Git
You can also configure Git to use main as the default branch locally:

Open Your Terminal: Run the following command to set main as the default branch name for new repositories:

sh
git config --global init.defaultBranch main
By configuring both GitHub and your local Git setup, you'll ensure that main is always used as the default branch for new repositories, avoiding the use of master.

This setup should save you time and streamline your workflow. 😊✨

Anything else you need help with?

is there anything in rust we can create to directly benefit github?
Absolutely! There are several ways Rust can directly benefit GitHub, ranging from enhancing developer tools to creating efficient automation scripts. Here are a few project ideas that you could consider:

Project Ideas for GitHub with Rust:
GitHub Action:

Create a custom GitHub Action using Rust to automate tasks like running tests, deploying applications, or managing pull requests. Rust’s performance and safety make it a great choice for building reliable actions.

CLI Tools:

Develop command-line tools to interact with the GitHub API. For instance, a tool that automates repetitive tasks such as managing issues, pull requests, or releases. The octocrab crate can be helpful here.

Bot for Issue Management:

Build a bot using Rust that automatically labels issues, assigns them to team members, or provides responses based on issue content. This can help streamline issue management and improve response times.

Metrics Dashboard:

Create a tool that collects and visualizes GitHub metrics like issue resolution times, pull request activity, and contributor statistics. This can be beneficial for project management and monitoring.

Example: GitHub Issue Bot
Here’s a simple example of a Rust application that interacts with the GitHub API to label issues based on their content:

Step-by-Step Implementation
Set Up Your Project:

sh
cargo new github_issue_bot
cd github_issue_bot
Add Dependencies: Add the necessary dependencies to your Cargo.toml:

toml
[dependencies]
octocrab = "0.13"
tokio = { version = "1", features = ["full"] }
Edit src/main.rs: Implement a basic bot that labels issues:

rust
use octocrab::{Octocrab, models::Issue};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Octocrab client
    let octocrab = Octocrab::builder().personal_token("YOUR_GITHUB_TOKEN").build()?;

    // Fetch issues from a repository
    let issues: Vec<Issue> = octocrab
        .issues("owner", "repo")
        .list()
        .send()
        .await?
        .items;

    // Process each issue
    for issue in issues {
        if issue.title.contains("bug") {
            // Add a label to the issue
            octocrab
                .issues("owner", "repo")
                .add_labels(issue.number, &["bug"])
                .await?;
        }
    }

    Ok(())
}
Running the Bot
Build and Run:

sh
cargo build --release
./target/release/github_issue_bot
GitHub Token: Make sure to replace YOUR_GITHUB_TOKEN with a personal access token from your GitHub account with appropriate permissions.

This example shows how you can create a bot that labels issues containing the word "bug". You can extend this further to handle more complex workflows.

README
Here's a sample README for this bot:

markdown
# GitHub Issue Bot

## Overview

`github_issue_bot` is a simple Rust application that interacts with the GitHub API to automate issue labeling based on the content of issue titles. This bot helps streamline issue management for your repositories.

## Features

- Automatically labels issues containing specific keywords
- Simple and easy-to-extend codebase
- Uses Rust for performance and reliability

## Installation

Clone the repository and navigate to the project directory:

```sh
git clone https://github.com/yourusername/github_issue_bot
cd github_issue_bot
Usage
Setting Up
Replace YOUR_GITHUB_TOKEN in src/main.rs with your personal access token.

Modify the repository owner and name in the code to match your repository.

Running the Bot
sh
cargo build --release
./target/release/github_issue_bot
Contributing
Contributions are welcome! Feel free to open issues or submit pull requests.

License
This project is licensed under the MIT License. See the LICENSE file for details.

Contact
For any questions, reach out to Your Name at your.email@example.com.

Happy coding! 😊✨


I hope this inspires you to build something amazing! If you need any further assistance or tweaks, let me know! 🚀✨
explain github actions to me in a way i can understand
Sure thing! Think of GitHub Actions as a way to automate tasks and workflows directly within your GitHub repository. It's like having a personal assistant that takes care of repetitive tasks for you, so you can focus on more important things. Here’s a simple breakdown:

What Are GitHub Actions?
GitHub Actions allows you to automate tasks such as building, testing, and deploying your code. These tasks are called "actions" and can be combined into workflows. Workflows are triggered by specific events in your repository, like pushing code, opening pull requests, or even on a schedule.

Key Components
Workflows:

Definition: A workflow is a series of actions defined in a .yml file within your .github/workflows directory.

Purpose: Automates tasks when certain events occur (e.g., code push, pull request).

Actions:

Definition: An individual task that runs during a workflow. Actions can be custom scripts, Docker containers, or published actions from the GitHub Marketplace.

Purpose: Executes specific steps like running tests, building code, or deploying applications.

Events:

Definition: Events are triggers that start workflows, such as push, pull_request, or schedule.

Purpose: Initiates the workflow when the specified event occurs in the repository.

Jobs:

Definition: A job is a set of steps that runs on the same runner. A workflow can have multiple jobs.

Purpose: Organizes actions into logical groups that execute on the same environment.

Runners:

Definition: Machines that run your workflows. GitHub provides hosted runners, but you can also use self-hosted runners.

Purpose: Executes the jobs defined in your workflow.

Simple Example
Imagine you want to automatically run tests every time you push code to your repository. Here’s a basic example of a GitHub Actions workflow that does that:

Create Workflow File:

Create a file named .github/workflows/test.yml in your repository.

Define Workflow:

yaml
name: CI

on: [push]

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
    - name: Checkout code
      uses: actions/checkout@v2

    - name: Set up Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable

    - name: Build
      run: cargo build --verbose

    - name: Run tests
      run: cargo test --verbose
Explanation
name: CI: The name of the workflow.

on: [push]: This workflow runs every time code is pushed to the repository.

jobs: Defines a job named build.

runs-on: ubuntu-latest: The job runs on the latest version of Ubuntu provided by GitHub.

steps: The individual actions to perform within the job:

Checkout code: Uses the actions/checkout@v2 action to check out the repository code.

Set up Rust: Sets up the Rust toolchain.

Build: Runs the build command.

Run tests: Runs the test command.

This is just a simple example, but GitHub Actions can do much more, including deploying applications, sending notifications, and integrating with other services.

I hope this makes GitHub Actions clearer! If you have more questions or want to see another example, just let me know. 😊🚀✨

back to pwm gen - i want to specify what sold ME on the idea to create this which was your idea - that it can produce high and low signals for GPIO pins - isn't this a good tool for embedded? pi? sbc's? riscv? arm?
Absolutely! A PWM signal generator is incredibly useful for a variety of embedded systems and single-board computers (SBCs) like Raspberry Pi, BeagleBone, and microcontrollers based on ARM or RISC-V architectures.

Why It's a Great Tool for Embedded Systems:
GPIO Pin Control:

PWM can directly control GPIO pins, making it perfect for embedded systems. It allows you to control devices like LEDs, motors, and servos with precise timing.

Versatility:

It's applicable across a wide range of platforms including Raspberry Pi (Pi), BeagleBone, and microcontrollers using ARM (like STM32) or RISC-V.

Educational Value:

Building a PWM generator is a great way to learn about timing, control, and signal generation, which are fundamental concepts in embedded systems.

Real-World Applications:

It can be used in numerous projects such as dimming LEDs, controlling motor speeds, generating audio signals, and more. This makes it a highly practical tool.

Extending the PWM Generator for Embedded Systems
To make your PWM generator control GPIO pins, you'll typically use a library specific to the hardware platform. Here are a few examples:

Raspberry Pi: Using the rppal crate for GPIO control.

STM32 (ARM Cortex-M): Using the stm32f4xx-hal crate.

General Embedded Rust: The embedded-hal crate provides a hardware abstraction layer for embedded devices.

Example with Raspberry Pi
Here’s how you can extend the PWM generator to control GPIO pins on a Raspberry Pi:

Add Dependencies: Add the rppal crate to your Cargo.toml:

toml
[dependencies]
rppal = "0.12"
Edit src/main.rs: Modify the code to control a GPIO pin:

rust
use std::thread;
use std::time::Duration;
use rppal::gpio::{Gpio, OutputPin};

struct Pwm {
    pin: OutputPin,
    duty_cycle: f32, // Duty cycle as a percentage (0.0 to 1.0)
    frequency: u32,  // Frequency in Hz
}

impl Pwm {
    fn new(pin: OutputPin, duty_cycle: f32, frequency: u32) -> Self {
        Pwm { pin, duty_cycle, frequency }
    }

    fn start(&mut self) {
        let period = Duration::from_micros((1_000_000 / self.frequency) as u64);
        let high_time = period.mul_f32(self.duty_cycle);
        let low_time = period - high_time;

        loop {
            // High signal
            self.pin.set_high();
            thread::sleep(high_time);

            // Low signal
            self.pin.set_low();
            thread::sleep(low_time);
        }
    }
}

fn main() {
    // Initialize GPIO and set pin to output mode
    let gpio = Gpio::new().expect("Failed to initialize GPIO");
    let pin = gpio.get(17).expect("Failed to get pin").into_output();

    let mut pwm = Pwm::new(pin, 0.5, 1000); // 50% duty cycle, 1kHz frequency
    pwm.start();
}
In this code, pin.set_high() and pin.set_low() control the GPIO pin’s state, generating the PWM signal.

This example is specific to Raspberry Pi, but similar principles apply to other platforms. Using embedded Rust libraries tailored for different hardware, you can extend this concept to a variety of systems.

Would you like to dive deeper into this or explore another aspect? 😊🚀✨

great - i may add these features or create a second version - but for ours AS IS right now, what can it do - it cannot do these things yet
Absolutely, for the current version of our PWM generator, here’s a clear breakdown of what it can do and what it cannot do:

Capabilities (As Is):
Simulate PWM Signals:

The program simulates PWM signals by toggling between "HIGH" and "LOW" states printed in the console.

Configurable duty cycle and frequency for the simulated signals.

Limitations:
No GPIO Control:

It does not currently control actual GPIO pins on any hardware platforms like Raspberry Pi, microcontrollers, or SBCs.

Console-Based Simulation:

The output is limited to the console, making it more of an educational tool to understand PWM rather than a practical tool for embedded applications.

No Integration with Hardware:

It doesn’t interface with embedded libraries or hardware abstraction layers that would allow it to run on specific devices such as ARM or RISC-V microcontrollers.

Potential Future Enhancements:
GPIO Pin Control:

Using libraries like rppal for Raspberry Pi or embedded-hal for general embedded development to actually control physical GPIO pins.

Support for Multiple Platforms:

Adding support for different hardware platforms, making it useful for a wider range of embedded systems.

User Interface:

Possibly adding a simple user interface for setting duty cycle and frequency parameters dynamically.

For now, the primary value of our PWM generator lies in its educational utility, helping users understand the basics of PWM through console output. It lays a strong foundation for more advanced features and real-world applications in future versions.

If you decide to expand its capabilities, it could become a versatile tool for embedded system enthusiasts and developers. Let me know if you need more guidance or ideas! 😊🚀✨

for the current version of our PWM generator, here’s a clear breakdown of what it can do and what it cannot do: Capabilities (As Is): Simulate PWM Signals: The program simulates PWM signals by toggling between "HIGH" and "LOW" states printed in the console. Configurable duty cycle and frequency for the simulated signals. Limitations: No GPIO Control: It does not currently control actual GPIO pins on any hardware platforms like Raspberry Pi, microcontrollers, or SBCs. Console-Based Simulation: The output is limited to the console, making it more of an educational tool to understand PWM rather than a practical tool for embedded applications. No Integration with Hardware: It doesn’t interface with embedded libraries or hardware abstraction layers that would allow it to run on specific devices such as ARM or RISC-V microcontrollers. Potential Future Enhancements: GPIO Pin Control: Using libraries like rppal for Raspberry Pi or embedded-hal for general embedded development to actually control physical GPIO pins. Support for Multiple Platforms: Adding support for different hardware platforms, making it useful for a wider range of embedded systems. User Interface: Possibly adding a simple user interface for setting duty cycle and frequency parameters dynamically. For now, the primary value of our PWM generator lies in its educational utility, helping users understand the basics of PWM through console output. It lays a strong foundation for more advanced features and real-world applications in future versions. If you decide to expand its capabilities, it could become a versatile tool for embedded system enthusiasts and developers. Let me know if you need more guidance or ideas! 😊🚀✨ - give this to me in under 30 words
The PWM generator simulates PWM signals via console output. It doesn't control GPIO pins or integrate with hardware. Ideal for educational purposes, with potential for future enhancements. 🌟

## Extending the PWM Generator for Embedded Systems To make your PWM generator control GPIO pins, you'll typically use a library specific to the hardware platform: Raspberry Pi: Using the rppal crate for GPIO control. STM32 (ARM Cortex-M): Using the stm32f4xx-hal crate. General Embedded Rust: The embedded-hal crate provides a hardware abstraction layer for embedded devices. - convert to .md for me so sentences don't run together
Sure, here's your text in markdown format with proper bullet points:

markdown
## Extending the PWM Generator for Embedded Systems

To make your PWM generator control GPIO pins, you'll typically use a library specific to the hardware platform:

- **Raspberry Pi**: Using the `rppal` crate for GPIO control.
- **STM32 (ARM Cortex-M)**: Using the `stm32f4xx-hal` crate.
- **General Embedded Rust**: The `embedded-hal` crate provides a hardware abstraction layer for embedded devices.
This should make it clearer and easier to read. Let me know if there's anything else you need! 😊

one-liner_stats: A small library that computes descriptive statistics (mean, median, mode, etc.) on the fly for numeric slices or streams. create in Rust?
Absolutely, let's create a small Rust library to compute descriptive statistics on the fly. Here's a step-by-step guide to get you started:

1. Set Up Your Project
First, create a new Rust library project:

sh
cargo new one_liner_stats --lib
cd one_liner_stats
2. Edit Cargo.toml
Make sure your Cargo.toml is set up correctly:

toml
[package]
name = "one_liner_stats"
version = "0.1.0"
authors = ["Your Name <your.email@example.com>"]
edition = "2021"

[dependencies]
3. Implement the Library
Edit src/lib.rs to include the functionality for computing mean, median, and mode:

rust
pub fn mean(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().sum();
    sum / data.len() as f64
}

pub fn median(data: &mut [f64]) -> f64 {
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = data.len();
    if len % 2 == 0 {
        (data[len / 2 - 1] + data[len / 2]) / 2.0
    } else {
        data[len / 2]
    }
}

pub fn mode(data: &[f64]) -> Option<f64> {
    use std::collections::HashMap;

    let mut occurrences = HashMap::new();

    for &value in data {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
}
4. Usage Example
You can add a small usage example in src/lib.rs to show how it works:

rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(mean(&data), 3.0);
    }

    #[test]
    fn test_median() {
        let mut data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(median(&mut data), 3.0);

        let mut data_even = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        assert_eq!(median(&mut data_even), 3.5);
    }

    #[test]
    fn test_mode() {
        let data = vec![1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 5.0];
        assert_eq!(mode(&data), Some(3.0));

        let data_no_mode = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(mode(&data_no_mode), Some(1.0)); // Or handle differently if needed
    }
}
5. Build and Test
Run the following commands to build and test your library:

sh
cargo build
cargo test
README
Here’s a sample README for your library:

markdown
# one_liner_stats

## Overview

`one_liner_stats` is a Rust library that computes descriptive statistics (mean, median, mode, etc.) on the fly for numeric slices or streams.

## Features

- Calculate mean, median, and mode for a given dataset.
- Simple and efficient implementation.

## Usage

### Add Dependency

Add `one_liner_stats` to your `Cargo.toml`:

```toml
[dependencies]
one_liner_stats = "0.1.0"
Example
rust
use one_liner_stats::{mean, median, mode};

fn main() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    let mean_value = mean(&data);
    let mut data_for_median = data.clone();
    let median_value = median(&mut data_for_median);
    let mode_value = mode(&data);

    println!("Mean: {}", mean_value);
    println!("Median: {}", median_value);
    println!("Mode: {:?}", mode_value);
}
```
## License
This project is licensed under the MIT License

##Author 
bensatlantik

