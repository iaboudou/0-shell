// change the current working directory.
pub fn run(args: &[String]) {
    if args.len() > 2 {
        eprintln!("cd: too many arguments");
        return;
    }

    let arg = if args.is_empty() || args[0] == "~" {
        match std::env::var("HOME") {
            Ok(home) => home,
            Err(e) => {
                eprintln!("cd: HOME not set: {}", e);
                return;
            }
        }
    } else if args[0] == "--" {
        if args.len() == 1 {
            match std::env::var("HOME") {
                Ok(home) => home,
                Err(e) => {
                    eprintln!("cd: HOME not set: {}", e);
                    return;
                }
            }
        } else {
            crate::commands::help::tilda(args[1].clone())
        }
    } else if args[0] == "-" {
        match std::env::var("OLDPWD") {
            Ok(oldpwd) => {
                println!("{}", oldpwd);
                oldpwd
            }
            Err(e) => {
                eprintln!("cd: OLDPWD not set: {}", e);
                return;
            }
        }
    } else {
        crate::commands::help::tilda(args[0].clone())
    };

    let oldpwd = std::env::current_dir();

    if let Err(er) = std::env::set_current_dir(&arg) {
        eprintln!("cd: {}: {}", arg, er.kind());
        return;
    }

    if let Ok(old) = oldpwd {
        unsafe { std::env::set_var("OLDPWD", old); }
    }
    if let Ok(new_dir) = std::env::current_dir() {
        unsafe { std::env::set_var("PWD", new_dir); }
    }
}
