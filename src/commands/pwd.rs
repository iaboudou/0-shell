
// Print the current working directory.
pub fn run(args: &[String]) {
    if !args.is_empty() {
        eprintln!("pwd: too many arguments");
        return;
    }

    match std::env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(_) => eprintln!("pwd: getcwd: No such file or directory"),
    }
}
