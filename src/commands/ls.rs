#[derive(Debug)]
struct Flags {
    l: bool,
    a: bool,
    f: bool,
}

#[derive(Debug)]
struct Ls {
    permission: String,
    hard_link: u64,
    owner: String,
    group: String,
    size: u64,
    last_update: String,
    name: String,
}

pub fn run(args: &[String]) {

    let mut flags = Flags{ l: false, a:false, f:false };
    let mut entries = vec!();

    if args.is_empty() {
        ls(".", &flags);
        return;
    }

    if !flags_and_args_are_correct(args, &mut flags, &mut entries) {
        usage();
        return;
    }

    for e in entries {
        ls(&e, &flags);
    }
}

// list a file or all entries in a directory
fn ls(path: &str, flags: &Flags) {

    let p = std::path::Path::new(path);
    if p.is_file() {
        print_entry(path, flags);
        return;
    }

    let entries = collect_directories(path.to_string());
    for e in entries {
        print_entry(&e, flags);
    }
} 

// collect and print information for a single file or directory entry
use std::os::unix::fs::PermissionsExt;
fn print_entry(entry: &str, flags: &Flags) {
    let mut ls = Ls {
        permission: "".to_owned(),
        hard_link: 0,
        owner: "".to_owned(),
        group: "".to_owned(),
        size: 0,
        last_update: "".to_owned(),
        name: "".to_owned(),
    };

    if flags.l {
        let path = std::path::Path::new(entry);
        ls.name = path.file_name().unwrap().to_str().unwrap().to_string();
        
        match std::fs::symlink_metadata(path) {
            Ok(meta_data) => {
                let p = format!("{:o}", meta_data.permissions().mode());
                let a = oct_to_string_permission(p);

                println!("{:?}", a);
            },
            Err(e) => {

            },
        }
    }
}

// validate all arguments and collect flags and paths
fn flags_and_args_are_correct(args: &[String], flags: &mut Flags, entries: &mut Vec<String>) -> bool {
    
    for arg in args {
        if arg.starts_with('-') {
            if !has_correct_flags(arg, flags) {
                return false;
            }
        } else {
            entries.push(arg.to_string());
        }
    }
    true
}

// check if all option flags are valid and store them
fn has_correct_flags(arg: &str, flags: &mut Flags) -> bool {

    if arg.len() <= 1 {
        return false;
    }

    for c in arg.chars().skip(1) {
        match c {
            'l' => flags.l = true,
            'F' => flags.f = true,
            'a' => flags.a = true,
            _ => {
                return false;
            }
        }
    }
    true
}


// collect all entries from a directory and return them sorted
fn collect_directories(path: String) -> Vec<String> {
    let mut entries = Vec::new();
    match std::fs::read_dir(path) {
        Ok(dir) => {
            for e in dir {
                entries.push(e.unwrap().path().display().to_string());
            }
        },
        Err(e) => {
            eprintln!("ls: {}", e);
        },
    }
    entries.sort();
    entries
}

// print the command usage
fn usage() {
    eprintln!("Usage: ls [-alF] [FILE]...");
}

// convert unix file mode to a human-readable permission string
fn oct_to_string_permission(per: String) -> String {
    let (typee, perm) = if per.len() == 5 {
        (&per[..2], &per[2..])
    } else {
        (&per[..3], &per[3..])
    };

    let perms = ["---", "--x", "-w-", "-wx", "r--", "r-x", "rw-", "rwx"];

    let file_type = match typee {
        "100" => "-",
        "40" => "d",
        "120" => "l",
        "60" => "b",
        "20" => "c",
        "10" => "p",
        "140" => "s",
        _ => "?",
    };

    let chars: Vec<char> = perm.chars().collect();

    format!(
        "{}{}{}{}",
        file_type,
        perms[chars[0].to_digit(8).unwrap() as usize],
        perms[chars[1].to_digit(8).unwrap() as usize],
        perms[chars[2].to_digit(8).unwrap() as usize],
    )
}