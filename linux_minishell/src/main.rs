use std::env;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::collections::VecDeque;
use colored::Colorize;
use dirs_next::home_dir;

// Constants
const HISTORY_CAPACITY: usize = 100;

// Struct to hold shell state
struct MiniShell {
    history: VecDeque<String>,
}

impl MiniShell {
    fn new() -> Self {
        MiniShell {
            history: VecDeque::with_capacity(HISTORY_CAPACITY),
        }
    }

    fn run(&mut self) {
        loop {
            self.show_prompt();
            let input = self.read_input();

            if input.trim().is_empty() {
                continue;
            }

            self.history.push_back(input.clone());
            if self.history.len() > HISTORY_CAPACITY {
                self.history.pop_front();
            }

            // Split input into command and arguments
            let mut parts = input.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args: Vec<&str> = parts.collect();

            // Handle built-in commands or run system commands
            match command {
                "exit" => break,
                "clear" => Self::clear_screen(),
                "pwd" => Self::print_working_directory(),
                "cd" => Self::change_directory(args),
                "history" => self.show_history(),
                _ => self.run_system_command(command, &args),
            }
        }
    }

    fn show_prompt(&self) {
        let current_dir = env::current_dir().unwrap();
        let prompt = format!("{}> ", current_dir.display());
        print!("{}", prompt.green().bold());
        io::stdout().flush().unwrap();
    }

    fn read_input(&self) -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().to_string()
    }

    fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn print_working_directory() {
        let current_dir = env::current_dir().unwrap();
        println!("{}", current_dir.display());
    }

    fn change_directory(args: Vec<&str>) {
        let new_dir = if args.is_empty() {
            home_dir().unwrap_or(env::current_dir().unwrap())
        } else {
            std::path::PathBuf::from(args[0])
        };

        if let Err(e) = env::set_current_dir(&new_dir) {
            eprintln!("cd: {}", e);
        }
    }

    fn show_history(&self) {
        for (index, command) in self.history.iter().enumerate() {
            println!("{}: {}", index + 1, command);
        }
    }

    fn run_system_command(&self, command: &str, args: &[&str]) {
        let child = Command::new(command)
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn();

        match child {
            Ok(mut child) => {
                let _ = child.wait();
            }
            Err(_) => {
                eprintln!("minishell: command not found: {}", command);
            }
        }
    }
}

fn main() {
    let mut shell = MiniShell::new();
    shell.run();
}
