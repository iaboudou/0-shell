use std::io::Write;

pub fn run(args: &[String]) {
    
    for (i, v) in args.iter().enumerate() {
        let v = &v.replace("\\n", "\n").replace("\\t", "\t").replace("\\\\", "\\");
        if i == args.len() - 1 {
            print!("{v}");
        }else {
            print!("{v} ");
        }
        _ = std::io::stdout().flush();
    }
    println!();
}
