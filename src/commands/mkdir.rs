
pub fn run(args: &[String]) {
    
    if args.is_empty() {
        eprintln!("mkdir: missing operand");
        return;
    }

    for arg in args {
        if let Err(e) = std::fs::create_dir(arg) {
            eprintln!("mkdir: {}", e);
        }
    }
}