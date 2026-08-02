// Change the current working directory.
pub fn run(args: &[String]) {

    if args.len() > 1 {
        eprintln!("cd: too many arguments");
        return;
    }

    if args.is_empty() || args[0] == "~" {
        match std::env::var("HOME") {
            Ok(home) => {
                let oldpwd = std::env::current_dir();

                if let Err(e) = std::env::set_current_dir(&home) {
                    eprintln!("cd: {}: {}", home, e.kind());
                    return;
                }

                if let Ok(old) = oldpwd {
                    unsafe { std::env::set_var("OLDPWD", old); }
                }
                unsafe { std::env::set_var("PWD", home); }
                
            }
            Err(e) => {
                eprintln!("cd: HOME not set: {}", e);
            }
        }
        return;
    }

    if args[0] == "-" {
        match std::env::var("OLDPWD") {
            Ok(oldpwd) => {
                let current = std::env::current_dir();

                if let Err(e) = std::env::set_current_dir(&oldpwd) {
                    eprintln!("cd: {}: {}", oldpwd, e.kind());
                    return;
                }

                if let Ok(cur) = current {
                    unsafe { std::env::set_var("OLDPWD", cur); }
                }
                unsafe { std::env::set_var("PWD", &oldpwd); }
                println!("{}", oldpwd);
            }
            Err(e) => {
                eprintln!("cd: OLDPWD not set: {}", e);
            }
        }
        return;
    }

    let arg = if args[0].starts_with("~/") {
        match std::env::var("HOME") {
            Ok(home) => format!("{}/{}", home, &args[0][2..]),
            Err(e) => {
                eprintln!("cd: HOME not set: {}", e);
                return;
            }
        }
    } else {
        args[0].clone()
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