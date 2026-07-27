

pub fn run(args: &[String]) {
    if args.is_empty() {
        let home = std::env::var("HOME").unwrap();
        if let Err(er) = std::env::set_current_dir(home) {
            eprintln!("cd: {}", er);
        }
        return;
    }

    if args.len() != 1 {
        println!("cd: too many arguments");
        return;
    }

    if let Err(er) = std::env::set_current_dir(&args[0]) {
        eprintln!("cd: {}", er);
    }
}