 #[cfg(not(unix))]
compile_error!("YOM is available on UNIX-like systems only.");

use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod builtins;

fn main() {
    let mut args = std::env::args_os();
    args.next();
    
    let path = match args.next() {
        Some(p) => p,
        None => return,
    };
    let f = File::open(&path).unwrap();
    
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let stdout = io::stdout();
    let mut lock = stdout.lock();
    while reader.read_line(&mut line).unwrap() > 0 {
        if line.starts_with("echo") {
            let str = line.trim_start_matches("echo ");
            let str = str.trim_end_matches("\n");
            if str.starts_with('"') && str.ends_with('"') {
                let str = str.trim_start_matches('"');
                let str = str.trim_end_matches('"');
                builtins::echo::main(str, &mut lock);
            } else {
                builtins::echo::main(str, &mut lock);
            }
        } else if line.starts_with("pwd") {
            let _ = builtins::pwd::main(&mut lock);
        } else if line.starts_with("cd") {
            let dir = line.trim_start_matches("cd ");
            let dir = dir.trim_end_matches("\n");
            if dir.starts_with('"') {
                let dir = dir.trim_start_matches('"');
                let dir = dir.trim_end_matches('"');
                let _ = builtins::cd::main(dir);
            } else {
                let _ = builtins::cd::main(dir);
            }
        }
        line.clear();
    }
    return;
}
