
pub fn run(args: &[String]) {

    if args.is_empty() || (args.len() == 1 && args[0] == "-r") {
        eprintln!("rm: missing operand");
        return;
    }

    let r_arg_exits = args[0] == "-r";
    let t = if r_arg_exits { &args[1..] } else { args };

    for arg in t {

        if r_arg_exits {
            if arg == "." || arg == ".." {
                eprintln!("rm: refusing to remove '.' or '..' directory: skipping '{}'", arg);
                continue;
            }

            if arg == "/" {
                eprintln!("rm: it is dangerous to operate recursively on '/'");
                continue;
            }
        }

        let metadata = match std::fs::symlink_metadata(arg) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("rm: cannot remove '{}': {}", arg, e);
                continue;
            }
        };

        let is_dir = metadata.is_dir();

        if is_dir {
            if !r_arg_exits {
                eprintln!("rm: cannot remove '{}': Is a directory", arg);
                continue;
            }

            if let Err(e) = std::fs::remove_dir_all(arg) {
                eprintln!("rm: cannot remove '{}': {}", arg, e);
            }
        } else {
            if let Err(e) = std::fs::remove_file(arg) {
                eprintln!("rm: cannot remove '{}': {}", arg, e);
            }
        }
    }
    
}