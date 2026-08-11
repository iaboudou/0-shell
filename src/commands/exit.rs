
pub fn run(args: &[String]) {
    let mut code = 0;

    if !args.is_empty() {
        code = match args[0].parse::<i128>() {
            Ok(n) if n >= 0 => (n % 256) as i32,
            _ => {
                eprintln!("exit: illegal number: {}", args[0]);
                return;
            }
        };
    }
    std::process::exit(code);
}