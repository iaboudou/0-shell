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
            Ok(path) => {

                let mut p = path.display().to_string();
                if let Ok(home) = std::env::var("HOME") {
                    if path.display().to_string().starts_with(&home) {
                        p = p.trim_start_matches(&home).to_string();
                        p.insert(0, '~');
                    }
                }
                print!("\x1b[34m{}$ \x1b[0m", p)
            },
            Err(_) => {
                let _ = std::env::set_current_dir("/");
                continue
            },
        }

        if let Err(_) = std::io::stdout().flush() {
            return;
        }

        let mut line = String::new();

        match std::io::stdin().read_line(&mut line) {
            Ok(n) => {
                // line.contains("\x1b")
                if line.chars().any(|c| c.is_control() && c != '\n' && c != '\t') {
                    eprintln!("0-shell: bad pattern");
                    continue;
                }

                if line.starts_with("#") {
                    continue;
                }

                if n == 0 {
                    println!();
                    break;
                }
                else if n != 1 {
                    if let Some(tokens) = input(line) {
                        if !tokens.is_empty() {
                            commands::dispatch(&tokens[0], &tokens[1..]);
                        }
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
                    line.push('\n');
                    line.push_str(&next);
                }
            },
        }
    }
}
