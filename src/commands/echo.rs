use std::io::Write;

pub fn run(args: &[String]) {
    
    for (i, v) in args.iter().enumerate() {
        if i == args.len() - 1 {
            print!("{v}");
        }else {
            print!("{v} ");
        }
        _ = std::io::stdout().flush();
    }
    println!();
}