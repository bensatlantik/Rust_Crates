## Archived Repository

**This repository was a gift from me to the Rust and open-source community. It is no longer actively maintained and has been archived. Feel free to fork and continue development on your own.**

# linux_minishell

A minimal, Bash-like shell written in Rust with support for common commands like `pwd`, `cd`, `history`, `clear`, and more. This shell aims to be simple, lightweight, and easy to extend for educational purposes or personal use.

## 🚀 Features
- **Built-in Commands**:
  - `pwd`: Print the current working directory.
  - `cd <path>`: Change the directory (defaults to the home directory if no path is provided).
  - `history`: Show a list of previously executed commands.
  - `clear`: Clear the terminal screen.
  - `exit`: Exit the shell.
- **System Command Support**: Execute standard system commands like `ls`, `echo`, `cat`, etc.
- **Colored Prompt**: Displays the current working directory in green.

## 📦 Installation
To compile and run `linux_minishell`, you'll need Rust installed on your system. If Rust is not installed, you can get it from [rust-lang.org](https://rust-lang.org).

Clone the repository and build the project:
```bash
git clone https://github.com/ben/linux_minishell.git
cd linux_minishell
cargo build --release
```
## 🛠️ Usage
Run the shell using:
``` 
cargo run
```
## 📋 Built-in Commands

| Command   | Description                                      |
|-----------|--------------------------------------------------|
| `pwd`     | Print the current working directory              |
| `cd`      | Change the directory (defaults to home if none)  |
| `history` | Show a list of previously executed commands      |
| `clear`   | Clear the terminal screen                        |
| `exit`    | Exit the shell                                   |

## 🔧 Dependencies

| Crate        | Description                                       |
|--------------|---------------------------------------------------|
| `colored`    | For colored text in the prompt                    |
| `dirs-next`  | For accessing the home directory                  |

## License 
This project is licensed under the MIT License. 

## Author
Ben Santora <bensatlantik@gmail.com>
