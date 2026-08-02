
pub fn run(args: &[String]) {
    
    if args.is_empty() || (args.len() == 1 && args[0] == "-r") {
        eprintln!("rm: missing operand");
        return;
    }

    if args[0] == "-r" {
        for arg in &args[1..] {

            let is_dir = std::fs::symlink_metadata(arg).map(|m| m.is_dir()).unwrap_or(false);
            if !is_dir {
                if let Err(e) = std::fs::remove_file(arg) {
                    eprintln!("rm: {}", e);
                }
                continue;
            }

            if let Err(e) = std::fs::remove_dir_all(arg) {
                eprintln!("rm: {}", e);
            }
        }
    }
    else {
        for arg in args {

            let is_dir = std::fs::symlink_metadata(arg).map(|m| m.is_dir()).unwrap_or(false);
            if is_dir {
                eprintln!("rm: cannot remove {}: Is a directory", arg);
                continue;
            }

            if let Err(e) = std::fs::remove_file(arg) {
                eprintln!("rm: {}", e);
            }
        }
    }

}