
pub fn run(args: &[String]) {
    let mut r = false;
    let mut can_take_options = true;
    let mut files = Vec::new();

    // collect valid args
    for arg in args {
        if can_take_options && arg == "--" {
            can_take_options = false;
            continue;
        }

        if can_take_options && arg == "-r" {
            r = true;
            continue;
        }

        files.push(arg);
    }

    if files.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }

    for arg in files {
        if arg == "." || arg == ".." {
            eprintln!(
                "rm: refusing to remove '{}' directory: skipping '{}'",
                arg, arg
            );
            continue;
        }

        if arg == "/" {
            eprintln!("rm: it is dangerous to operate recursively on '/'");
            continue;
        }

        let metadata = match std::fs::symlink_metadata(arg) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("rm: cannot remove '{}': {}", arg, e.kind());
                continue;
            }
        };

        if metadata.is_dir() {
            if !r {
                eprintln!("rm: cannot remove '{}': Is a directory", arg);
                continue;
            }

            if let Err(e) = std::fs::remove_dir_all(arg) {
                eprintln!("rm: cannot remove '{}': {}", arg, e.kind());
            }
        } else if let Err(e) = std::fs::remove_file(arg) {
            eprintln!("rm: cannot remove '{}': {}", arg, e.kind());
        }
    }
}