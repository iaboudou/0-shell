
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

        let va = crate::commands::help::tilda(arg.to_string());

        if va == "/" {
            eprintln!("rm: it is dangerous to operate recursively on '/'");
            continue;
        }

        let metadata = match std::fs::symlink_metadata(&va) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("rm: cannot remove '{}': {}", va, e.kind());
                continue;
            }
        };

        if metadata.is_dir() {
            if !r {
                eprintln!("rm: cannot remove '{}': Is a directory", va);
                continue;
            }

            if let Err(e) = std::fs::remove_dir_all(&va) {
                eprintln!("rm: cannot remove '{}': {}", va, e.kind());
            }
        } else if let Err(e) = std::fs::remove_file(&va) {
            eprintln!("rm: cannot remove '{}': {}", va, e.kind());
        }
    }
}