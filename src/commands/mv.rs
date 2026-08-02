

pub fn run(args: &[String]) {
    if args.len() == 0 {
        eprintln!("mv: missing file operand");
        return;
    }
    else if args.len() == 1 {
        eprintln!("mv: missing destination file operand after {}", args[0]);
        return;
    }

    let dest = std::path::Path::new(&args[args.len() -1]);

    if args.len() > 2 && !dest.is_dir() { 
        eprintln!("mv: target '{}' is not a directory", args[args.len() - 1]); 
        return; 
    }

    for arg in &args[..args.len() - 1] {
        
        let src = std::path::Path::new(arg);
    
        if !src.exists() && !src.is_symlink() {
            eprintln!("mv: cannot stat '{}': No such file or directory", arg);
            continue;
        }
    
        // if both args are the same
        match (std::fs::canonicalize(src), std::fs::canonicalize(dest)) {
            (Ok(s), Ok(d)) => {
                if s == d {
                    eprintln!("mv: '{}' and '{}' are the same", arg, &args[args.len() - 1]);
                    continue;
                }
            },
            _ => {},
        }
    
        // 2nd arg is a dir
        if dest.is_dir() {
        
            let f_n = match src.file_name() {
                Some(name) => name,
                None => {
                    eprintln!("mv: cannot determine filename for '{}'", arg);
                    continue;
                }
            };
        
            if let Err(e) = std::fs::rename(src, &(dest.join(f_n))) {
                eprintln!("mv: {}", e);
            }
            continue;
        }
        
        // 2nd arg is file
        if let Err(e) = std::fs::rename(src, dest) {
            eprintln!("mv: {}", e);
        }
    }

}