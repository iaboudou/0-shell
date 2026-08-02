use std::io::Write;

// Display file contents or read from standard input.
pub fn run(args: &[String]) {

    if args.is_empty() {
        loop {
            let mut input = String::new();

            let n = match std::io::stdin().read_line(&mut input) {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("cat: {}", e);
                    break;
                }
            };

            if n == 0 {
                break;
            }
        
            print!("{input}");
        }
    }

    for file in args {
        match std::fs::read(file) {
            Ok(content) => {
                if let Err(e) = std::io::stdout().write_all(&content) {
                    eprintln!("cat: {}", e);
                }
            },
            Err(e) => {
                eprintln!("cat: {}: {}", file, e);
            }
        }
    }
}