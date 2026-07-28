
pub fn run(args: &[String]) {
    if args.len() < 2 {
        eprintln!("cp: missing file operand");
        return;
    }
    if args.len() > 2 {
        eprintln!("cp: too many arguments");
        return;
    }

    let src = std::path::Path::new(&args[0]);
    let dest = std::path::Path::new(&args[1]);

    
    if !src.is_file() {
        eprintln!("cp: '{}' is not a file", args[0]);
        return;
    }

    // if both args are the same file
    match (std::fs::canonicalize(src), std::fs::canonicalize(dest)) {
        (Ok(s), Ok(d)) => {
            if s == d {
                eprintln!("cp: '{}' and '{}' are the same file", args[0], args[1]);
                return;
            }
        },
        _ => {},
    }

    // 2nd arg is a dir
    if  dest.is_dir() {
        if let Err(e) = std::fs::copy(src, &(dest.join(src.file_name().unwrap()))) {
            eprintln!("cp: {}", e);
        }
        return;
    }
    
    // 2nd arg is file
    if let Err(e) = std::fs::copy(src, dest) {
        eprintln!("cp: {}", e);
    }

}