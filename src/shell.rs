use std::io::Write;
use crate::{ parser, commands };

pub enum ParseResult {
    Complete(Vec<String>),
    Uncomplete(String),
}

// Process user input and call the input function to handle incomplete commands
pub fn run() {
    loop {

        match std::env::current_dir() {
            Ok(path) => print!("{} $ ", path.display()),
            Err(_) => print!("$ "),
        }

        if let Err(_) = std::io::stdout().flush() {
            return;
        }

        let mut line = String::new();

        match std::io::stdin().read_line(&mut line) {
            Ok(n) => {
                if n == 0 {
                    println!();
                    break;
                }
                else if n != 1 {
                    if let Some(tokens) = input(line) {
                        commands::dispatch(&tokens[0], &tokens[1..]);
                    }
                }
            },
            Err(e) => eprintln!(": {}", e),
        }
    }
}

// Read additional input when quotes are not closed
fn input(mut line: String) -> Option<Vec<String>> {
    loop {
        if line.ends_with('\n') {
            line.pop();
        }

        match parser::parse(&line) {

            ParseResult::Complete(tokens) => {
                return Some(tokens);
            },

            ParseResult::Uncomplete(s) => {
                print!("{s}");

                if let Err(_) = std::io::stdout().flush() {
                    return None;
                }

                let mut next = String::new();
                let n = match std::io::stdin().read_line(&mut next) {
                    Ok(n) => n,
                    Err(_) => return None,
                };

                if n == 0 {
                    return None;
                }
                else {
                    if !next.trim().ends_with('\\') && line.starts_with("echo") && !next.trim().ends_with('"') {
                        line.push('\n');
                    }

                    if line.trim_end().ends_with('\\') {
                        line = line.trim_end().to_string();
                        line.pop();
                    }
                    line.push_str(&next);
                }
            },
        }
    }
}