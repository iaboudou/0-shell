use std::io::Write;

pub fn run(args: &[String]) {

    if args.is_empty() {
        loop {
            let mut input = String::new();
            let n = std::io::stdin().read_line(&mut input).unwrap();
            
            if n == 0 {
                print!("{input}");
                std::io::stdout().flush().unwrap();
                break;
            }

            print!("{input}");
        }
    }

    for file in args {
        match std::fs::read(file) {
            Ok(content) => {
                std::io::stdout().write_all(&content).unwrap();
            },
            Err(e) => {
                eprintln!("cat: {}: {}", file, e);
            }
        }
    }
}