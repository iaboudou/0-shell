mod echo;
mod cd;
mod ls;
mod pwd;
mod cat;
mod cp;
mod rm;
mod mv;
mod mkdir;
mod exit;
mod help;

// Find and execute the corresponding command
pub fn dispatch(cmd: &str, args: &[String]) {
    match cmd {
        "cat" => cat::run(args),
        "cd" => cd::run(args),
        "cp" => cp::run(args),
        "echo" => echo::run(args),
        "exit" => exit::run(args),
        "ls" => ls::run(args),
        "mkdir" => mkdir::run(args),
        "mv" => mv::run(args),
        "pwd" => pwd::run(args),
        "rm" => rm::run(args),
        "help" => help::help(),
        other => println!("Command '{}' not found", other),
    }
}
