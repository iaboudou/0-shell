pub fn run(args: &[String]) {
    if args.len() < 2 {
        eprintln!("cp: missing file operand");
        return;
    }

    let dest_str = crate::commands::help::tilda(args[args.len() - 1].clone());
    let dest = std::path::Path::new(&dest_str);

    if args.len() > 2 && !dest.is_dir() { 
        eprintln!("cp: target '{}' is not a directory", args[args.len() - 1]); 
        return; 
    }

    for arg in &args[..args.len() - 1] {
        let arg = crate::commands::help::tilda(arg.to_string());
        let src = std::path::Path::new(&arg);
        
        if !src.exists() && !src.is_symlink() {
            eprintln!("cp: cannot stat '{}': No such file or directory", arg);
            continue;
        }
    
        // if both args are the same file
        match (std::fs::canonicalize(src), std::fs::canonicalize(dest)) {
            (Ok(s), Ok(d)) => {
                if s == d {
                    eprintln!("cp: '{}' and '{}' are the same file", arg, &args[args.len() - 1]);
                    continue;
                }
            },
            _ => {},
        }
    
        // 2nd arg is a dir
        if  dest.is_dir() {

            let f_n = match src.file_name() {
                Some(name) => name,
                None => {
                    eprintln!("cp: cannot determine filename for '{}'", arg);
                    continue;
                }
            };

            if let Err(e) = std::fs::copy(src, &(dest.join(f_n))) {
                eprintln!("cp: {}: {}", arg, e);
            }
            continue;
        }
        
        // 2nd arg is file
        if let Err(e) = std::fs::copy(src, dest) {
            eprintln!("cp: {}: {}", arg, e.kind());
        }
    }
}
