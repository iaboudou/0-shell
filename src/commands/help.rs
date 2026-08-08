
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
