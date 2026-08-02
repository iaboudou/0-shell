use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::FileTypeExt;
use std::fmt::{Display, Formatter, Result};

pub fn run(args: &[String]) {

    let mut flags = Flags::new();
    let mut entries = vec!();

    if args.is_empty() {
        Ls::ls(".", &flags);
        return;
    }

    if !flags.flags_and_args_are_correct(args, &mut entries) {
        Ls::usage();
        return;
    }

    if entries.is_empty() {
        Ls::ls(".", &flags);
        return;
    }

    for e in entries {
        Ls::ls(&e, &flags);
    }
}

#[derive(Debug, Clone)]
struct Flags {
    l: bool,
    a: bool,
    f: bool,
}

impl Flags {
    fn new() -> Self {
        Flags { l: false, a: false, f: false }
    }

    // check if all option flags are valid and store them
    fn has_correct_flags(&mut self, arg: &str) -> bool {

        if arg.len() <= 1 {
            return false;
        }

        for c in arg.chars().skip(1) {
            match c {
                'l' => self.l = true,
                'F' => self.f = true,
                'a' => self.a = true,
                _ => {
                    return false;
                }
            }
        }
        true
    }

    // validate all arguments and collect flags and paths
    fn flags_and_args_are_correct(&mut self, args: &[String], entries: &mut Vec<String>) -> bool {

        for arg in args {
            if arg.starts_with('-') {
                if !self.has_correct_flags(arg) {
                    return false;
                }
            } else {
                entries.push(arg.to_string());
            }
        }
        true
    }
}

#[derive(Debug)]
struct Ls {
    permission: String,
    hard_link: u64,
    owner: String,
    group: String,
    size: String, // will be "{major, minor}" in case of block device or character device (c/b)
    last_update: String,
    name: String,
    flags : Flags,
}

impl Ls {
    fn new() -> Self {
        Self {
            permission: "".to_owned(),
            hard_link: 0,
            owner: "".to_owned(),
            group: "".to_owned(),
            size: "".to_owned(),
            last_update: "".to_owned(),
            name: "".to_owned(),
            flags: Flags::new(),
        }
    }

    // list a file or all entries in a directory
    fn ls(path: &str, flags: &Flags) {

        let p = std::path::Path::new(path);
        let meta_data = std::fs::symlink_metadata(p);
        
        // if path is file
        if let Ok(metadata) = meta_data {
            if metadata.file_type().is_file()
                    || metadata.file_type().is_symlink()
                    || metadata.file_type().is_fifo()
                    || metadata.file_type().is_socket()
                    || metadata.file_type().is_char_device()
                    || metadata.file_type().is_block_device()
                {
                    let mut ls = Self::new();
                    ls.flags = flags.clone();
                    ls.mutate_ls_info(path);
                    println!("{}", ls);
                    return;
                }
        }

        // if path is not a file
        let mut entries = match Self::collect_directories(path.to_string(), flags) {
            Some(e) => e,
            None => return,
        };
        if flags.a {
            entries.insert(0, format!("{path}/.."));
            entries.insert(0, format!("{path}/."));  
        }

        let mut total = 0;
        for e in &entries {
            if let Ok(metadata) = std::fs::symlink_metadata(e) {
                total += metadata.blocks() / 2;
            }
        }

        if flags.l {
            println!("total {}", total);
        }

        let is_empty = entries.is_empty();

        for e in entries {
            let mut ls = Self::new();
            ls.flags = flags.clone();
            ls.mutate_ls_info(&e);
        
            if flags.l {
                println!("{}", ls);
            } else {
                print!("{} ", ls);
            }
        }

        if !flags.l && !is_empty {
            println!();
        }
    }

    // collect and print information for a single file or directory entry
    fn mutate_ls_info(&mut self, entry: &str) {
   
        let path = std::path::Path::new(entry);

        self.name = if entry == ".." || entry.ends_with("/..") {
            "..".to_string()
        } else if entry == "." || entry.ends_with("/.") {
            ".".to_string()
        } else {
            match path.file_name() {
                Some(name) => name.to_string_lossy().to_string(),
                None => entry.to_string(),
            }
        };

        let sl_metadata = std::fs::symlink_metadata(path);

        if self.flags.l {
            match &sl_metadata {
                Ok(meta_data) => {

                    // permission
                    self.permission = Self::oct_to_string_permission( meta_data.permissions().mode() );

                    // hard link
                    self.hard_link = meta_data.nlink();

                    // owner
                    match users::get_user_by_uid(meta_data.uid()) {
                        Some(user) => {
                            self.owner = user.name().display().to_string();
                        }
                        None => {
                            self.owner = meta_data.uid().to_string();
                        }
                    }

                    // group
                    match users::get_group_by_gid(meta_data.gid()) {
                        Some(group) => {
                            self.group = group.name().to_string_lossy().to_string();
                        }
                        None => {
                            self.group = meta_data.gid().to_string();
                        }
                    }
                    
                    // in case of (c/b)
                    if meta_data.file_type().is_char_device() || meta_data.file_type().is_block_device() {
                        self.size = format!("{:>3}, {:>3}", libc::major(meta_data.rdev()), libc::minor(meta_data.rdev()));
                    }
                    // size
                    else {
                        self.size = meta_data.size().to_string();
                    }

                    // last update
                    match chrono::DateTime::from_timestamp(meta_data.mtime(), 0) {
                        Some(e) => {
                            let tz = e.with_timezone(&chrono::Local);
                            self.last_update = tz.format("%b %d %H:%M").to_string();
                        },
                        None => {
                            self.last_update = "????".to_string();
                        }
                    };

                    // add (-> target) if the file is a symlink
                   if meta_data.file_type().is_symlink() {
                        if let Ok(target) = std::fs::read_link(path) {
                            let mut t = target.display().to_string();
                        
                            if self.flags.f {
                                if let Ok(t_md) = std::fs::metadata(path) {
                                    t.push_str(Self::file_type_indicator(&t_md, &self.flags));
                                }
                            }
                            self.name.push_str(&format!(" -> {}", t));
                        }
                    }
                },
                Err(e) => {
                    eprintln!("ls: {}: {}", entry, e);
                },
            }
        }

        if self.flags.f {
            match &sl_metadata {
                Ok(meta_data) => {
                    self.name.push_str(Ls::file_type_indicator(meta_data, &self.flags));
                },
                Err(e) => {
                    eprintln!("ls: {}: {}", entry, e);
                },
            }
        }
    }

    // collect all entries from a directory and return them sorted
    fn collect_directories(path: String, flags: &Flags) -> Option<Vec<String>> {
        let mut entries = Vec::new();

        match std::fs::read_dir(&path) {
            Ok(dir) => {
                for entry in dir {
                    match entry {
                        Ok(e) => {
                            if flags.a || !e.file_name().to_string_lossy().starts_with('.') {
                                entries.push(e.path().display().to_string());
                            }
                        }
                        Err(e) => {
                            eprintln!("ls: {}: {}", path, e);
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("ls: cannot access '{}': {}", path, e);
                return None;
            },
        }
        entries.sort();
        Some(entries)
    }

    // convert unix file mode to a human-readable permission string
    fn oct_to_string_permission(mode: u32) -> String {

        let file_type = match mode & 0b1111_000_000_000_000 {
            0b1000_000_000_000_000 => "-",
            0b0100_000_000_000_000 => "d",
            0b1010_000_000_000_000 => "l",
            0b0110_000_000_000_000 => "b",
            0b0010_000_000_000_000 => "c",
            0b0001_000_000_000_000 => "p",
            0b1100_000_000_000_000 => "s",
            _ => "?",
        };

        let perms = ["---", "--x", "-w-", "-wx", "r--", "r-x", "rw-", "rwx"];

        let mut owner = perms[((mode >> 6) & 0b0000_000_000_000_111) as usize].to_owned();
        let mut group = perms[((mode >> 3) & 0b0000_000_000_000_111) as usize].to_owned();
        let mut other = perms[(mode & 0b0000_000_000_000_111) as usize].to_owned();

        // setuid -> replace with s/S
        if mode & 0b0000_100_000_000_000 != 0 {
            if mode & 0b0000_000_001_000_000 != 0 {
                owner.replace_range(2..3, "s");
            }else {
                owner.replace_range(2..3, "S");
            }
        }

        // setgid -> replace with s/S
        if mode & 0b0000_010_000_000_000 != 0 {
            if mode & 0b0000_000_000_001_000 != 0 {
                group.replace_range(2..3, "s");
            }else {
                group.replace_range(2..3, "S");
            }
        }

        // sticky bit -> replace with t/T
        if mode & 0b0000_001_000_000_000 != 0 {
            if mode & 0b0000_000_000_000_001 != 0 {
                other.replace_range(2..3, "t");
            }else {
                other.replace_range(2..3, "T");
            }
        }

        format!("{}{}{}{}", file_type, owner, group, other)
    }

    // take the suffix character based on the file's type
    fn file_type_indicator(metadata: &std::fs::Metadata, flags: &Flags) -> &'static str {
        let mode = metadata.permissions().mode();
    
        if metadata.file_type().is_symlink() {
            if flags.l {
                ""
            } else {
                "@"
            }
        } else if metadata.file_type().is_dir() {
            "/"
        } else if metadata.file_type().is_fifo() {
            "|"
        } else if metadata.file_type().is_socket() {
            "="
        } else if mode & 0o111 != 0 {
            "*"
        } else {
            ""
        }
    }

    // print the command usage
    fn usage() {
        eprintln!("Usage: ls [-alF] [FILE]...");
    }
}

impl Display for Ls {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.flags.l {
            write!(
                f,
                "{} {:>2} {:<10} {:<10} {:>11} {:>15} {}",
                self.permission,
                self.hard_link,
                self.owner,
                self.group,
                self.size,
                self.last_update,
                self.name
            )?;
        } else {
            write!(f, "{}", self.name)?;
        }

        Ok(())
    }
}