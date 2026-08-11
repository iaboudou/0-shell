use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::FileTypeExt;
use std::fmt::{Display, Formatter, Result};
use std::os::unix::ffi::OsStrExt;

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

    entries.sort_by_key(|e| { if std::path::Path::new(e).is_dir() { 1 } else { 0 } });

    for (i, e) in entries.iter().enumerate() {
        if entries.len() > 1 && std::path::Path::new(e).is_dir() {
            println!("{}:", e);
        }

        Ls::ls(e, &flags);

        if i + 1 < entries.len() {
            if std::path::Path::new(&entries[i + 1]).is_dir() {
                println!();
            }
        }
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
        let mut can_take_options = true;
        for arg in args {
            if arg == "--" {
                can_take_options = false;
                continue;
            }

            if arg.starts_with('-') && can_take_options {
                if !self.has_correct_flags(arg) {
                    return false;
                }
            } else {
                if arg == "~" {
                    match std::env::var("HOME") {
                        Ok(n) => {
                            entries.push(n);
                            continue;
                        }
                        _ => {}
                    }
                }
                entries.push(arg.to_string());
            }
        }
        true
    }
}

#[derive(Debug, Clone)]
struct Widths {
    permission: usize,
    hard_link: usize,
    owner: usize,
    group: usize,
    size: usize,
    last_update: usize,
    name: usize,
}

impl Widths {
    fn new() -> Self {
        Self { permission:0, hard_link: 0, owner: 0, group: 0, size: 0, last_update:0, name: 0, }
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
    name: std::ffi::OsString,
    flags : Flags,
    widths: Widths,
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
            name: std::ffi::OsString::new(),
            flags: Flags::new(),
            widths: Widths::new(),
        }
    }

    // list a file or all entries in a directory
    fn ls(path: &str, flags: &Flags) {

        let p = std::path::Path::new(path);
        let meta_data = std::fs::symlink_metadata(p);
        
        // if path is file
        if let Ok(metadata) = meta_data {
            if metadata.file_type().is_symlink() {
                if let Ok(target) = std::fs::metadata(p) {
                    if !target.is_dir() || flags.l {
                        let mut ls = Self::new();
                        ls.flags = flags.clone();
                        ls.mutate_ls_info(std::ffi::OsStr::new(path));
                        println!("{}", ls);
                        return;
                    }
                } else {
                    let mut ls = Self::new();
                    ls.flags = flags.clone();
                    ls.mutate_ls_info(std::ffi::OsStr::new(path));
                    println!("{}", ls);
                    return;
                }
            }
        
            if metadata.file_type().is_file()
                || metadata.file_type().is_fifo()
                || metadata.file_type().is_socket()
                || metadata.file_type().is_char_device()
                || metadata.file_type().is_block_device()
            {
                let mut ls = Self::new();
                ls.flags = flags.clone();
                ls.mutate_ls_info(std::ffi::OsStr::new(path));
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
            entries.insert(0, std::ffi::OsString::from(format!("{path}/..")));
            entries.insert(0, std::ffi::OsString::from(format!("{path}/.")));  
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

        let mut ls_list: Vec<Ls> = Vec::new();

        for e in entries {
            let mut ls = Self::new();
            ls.flags = flags.clone();
            ls.mutate_ls_info(&e);
            ls_list.push(ls);
        }

        let mut widths = Widths::new();
        for ls in &ls_list {
            widths.permission = widths.permission.max(ls.permission.len());
            widths.hard_link = widths.hard_link.max(ls.hard_link.to_string().len());
            widths.owner = widths.owner.max(ls.owner.len());
            widths.group = widths.group.max(ls.group.len());
            widths.size = widths.size.max(ls.size.len());
            widths.last_update = widths.last_update.max(ls.last_update.len());
            widths.name = widths.name.max(ls.name.len());
        }

        for ls in &mut ls_list {
            ls.widths = widths.clone();
            if flags.l {
                println!("{} ", ls);
            } else {
                print!("{} ", ls);
            }
        }

        if !flags.l && !is_empty {
            println!();
        }
    }

    // collect and print information for a single file or directory entry
    fn mutate_ls_info(&mut self, entry: &std::ffi::OsStr) {
   
        let path = std::path::Path::new(entry);

        let s = entry.to_string_lossy();
        self.name = if s == "." || s.ends_with("/.") {
            std::ffi::OsString::from(".")
        } else if s == ".." || s.ends_with("/..") {
            std::ffi::OsString::from("..")
        } else {
            match path.file_name() {
                Some(name) => name.to_os_string(),
                None => entry.to_os_string(),
            }
        };

        let sl_metadata = std::fs::symlink_metadata(path);

        if self.flags.l {
            match &sl_metadata {
                Ok(meta_data) => {

                    // permission
                    self.permission = Self::oct_to_string_permission( meta_data );

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
                            let mut t = target.as_os_str().to_os_string();
                        
                            if self.flags.f {
                                if let Ok(t_md) = std::fs::metadata(path) {
                                    t.push(Self::file_type_indicator(&t_md, &self.flags));
                                }
                            }
                            self.name.push(&format!(" -> {}", t.display()));
                        }
                    }
                },
                Err(e) => {
                    eprintln!("ls: {}: {}", entry.display(), e.kind());
                },
            }
        }

        if self.flags.f {
            match &sl_metadata {
                Ok(meta_data) => {
                    self.name.push(Ls::file_type_indicator(meta_data, &self.flags));
                },
                Err(e) => {
                    eprintln!("ls: {}: {}", entry.display(), e.kind());
                },
            }
        }
    }

    // collect all entries from a directory and return them sorted
    fn collect_directories(path: String, flags: &Flags) -> Option<Vec<std::ffi::OsString>> {
        let mut entries = Vec::new();

        match std::fs::read_dir(&path) {
            Ok(dir) => {
                for entry in dir {
                    match entry {
                        Ok(e) => {
                            let name = e.file_name();
                            if flags.a || !name.as_bytes().starts_with(b".") {
                                entries.push(e.path().into_os_string());
                            }
                        }
                        Err(e) => {
                            eprintln!("ls: {}: {}", path, e.kind());
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("ls: cannot access '{}': {}", path, e.kind());
                return None;
            },
        }

        entries.sort_by(|a, b| {
            let a = a.as_os_str().as_bytes();
            let b = b.as_os_str().as_bytes();
            a.cmp(b)
        });

        Some(entries)
    }

    // convert unix file mode to a human-readable permission string
    fn oct_to_string_permission(metadata: &std::fs::Metadata) -> String {

        let mode = metadata.permissions().mode();

        let file_type = metadata.file_type();
            
        let c = if file_type.is_dir() {
            "d"
        } else if file_type.is_symlink() {
            "l"
        } else if file_type.is_block_device() {
            "b"
        } else if file_type.is_char_device() {
            "c"
        } else if file_type.is_fifo() {
            "p"
        } else if file_type.is_socket() {
            "s"
        } else if file_type.is_file() {
            "-"
        } else {
            "?"
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

        format!("{}{}{}{}", c, owner, group, other)
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

    fn escape_name(name: &std::ffi::OsStr) -> String {
        let mut result = String::new();
        
        for c in name.to_string_lossy().chars() {
            if c.is_control() {
                result.push('?');
            } else {
                result.push(c);
            }
        }
    
        result
    }
}

impl Display for Ls {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.flags.l {

            let w_hard_link = self.widths.hard_link;
            let w_owner = self.widths.owner;
            let w_group = self.widths.group;
            let w_size = self.widths.size;
            let w_last_update = self.widths.last_update;

            write!(
                f,
                "{}  {:>w_hard_link$} {:<w_owner$} {:<w_group$}  {:>w_size$} {:>w_last_update$}  {}",
                self.permission,
                self.hard_link,
                self.owner,
                self.group,
                self.size,
                self.last_update,
                Ls::escape_name(&self.name)
            )?;
        } else {
            write!(f, "{}", Ls::escape_name(&self.name))?;
        }

        Ok(())
    }
}
