
pub fn run(args: &[String]) {
    let mut code = 0;

    if !args.is_empty() {

        if args[0].chars().any(|x| !x.is_ascii_digit()) {
            eprintln!("exit: illegal number: {}", args[0]);
            return;
        }

        code = match args[0].parse::<i128>() {
            Ok(n) if n >= 0 => (n % 256) as i32,
            _ => {
                255
            }
        };
    }
    std::process::exit(code);
}