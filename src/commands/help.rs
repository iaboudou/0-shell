
pub fn help() {
    println!("Built-in commands:");
    println!();
    println!("echo [STRING...]");
    println!("  Display STRING...");
    println!();
    println!("cd [DIR]");
    println!("  Change the current directory");
    println!();
    println!("ls [-l] [-a] [-F] [PATH]");
    println!("  -l    long listing format");
    println!("  -a    show hidden files");
    println!("  -F    append indicator to entries");
    println!();
    println!("pwd");
    println!("  Print the current working directory");
    println!();
    println!("cat [FILE...]");
    println!("  Display file contents");
    println!();
    println!("cp SOURCE DEST");
    println!("  Copy SOURCE to DEST");
    println!();
    println!("rm [-r] FILE...");
    println!("  -r    remove directories recursively");
    println!();
    println!("mv SOURCE DEST");
    println!("  Move SOURCE to DEST");
    println!();
    println!("mkdir [DIR...]");
    println!("  Create directories");
    println!();
    println!("exit");
    println!("  Exit the shell");
}

// replace ~/ with the user's HOME directory.
pub fn tilda(arg: String) -> String {
    if arg.starts_with("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return format!("{}{}", home, &arg[1..]);
        }
    }
    arg
}

// check if the path is dangerous to remove.
pub fn is_dangerous(path: &str) -> (bool, String) {
    if path == "/" {
        return ( true, "rm: it is dangerous to operate recursively on '/'".to_string());
    }

    let path = path.trim_end_matches('/');

    if path.ends_with("/..") || path == ".." || path.ends_with("/.") || path == "." {
        return (true, "rm: can't remove '.' or '..'".to_string());
    }

    (false, String::new())
}