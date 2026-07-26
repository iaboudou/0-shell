use std::io::Write;
use crate::parser;

pub fn run() {
    loop {
        print!("$ ");
        std::io::stdout().flush().unwrap();


        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(n) => {
                // ctrl + D
                if n == 0 {
                    println!();
                    break;
                }
                // command not empty
                else if n != 1 {
                    let _tockens = parser::parse(&line);
                }
            },
            Err(e) => eprintln!(": {}", e),
        }

    }
}