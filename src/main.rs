 #[cfg(not(unix))]
compile_error!("YOM is available on UNIX-like systems only.");

use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod builtins;
mod externals;
use std::process::exit;

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
        if line.starts_with("#") {
            line.clear();
            continue;
        } else if line.starts_with("echo ") {
            let str = line.trim_start_matches("echo ");
            let str = str.trim_end_matches("\n");
            if str.starts_with('"') && str.ends_with('"') {
                let str = str.trim_start_matches('"');
                let str = str.trim_end_matches('"');
                builtins::echo::echo(str, &mut lock);
            } else {
                builtins::echo::echo(str, &mut lock);
            }
        } else if line == "pwd\n" {
            let _ = builtins::pwd::pwd(&mut lock);
        } else if line.starts_with("cd ") {
            let dir = line.trim_start_matches("cd ");
            let dir = dir.trim_end_matches("\n");
            if dir.starts_with('"') && dir.ends_with('"') {
                let dir = dir.trim_start_matches('"');
                let dir = dir.trim_end_matches('"');
                let _ = builtins::cd::cd(dir);
            } else {
                let _ = builtins::cd::cd(dir);
            }
        } else if line == "exit\n" {
            let _ = builtins::exit::main(0);
        } else if line.starts_with("exit ") {
            let code: i32 = line.trim_start_matches("exit ").trim_end_matches("\n").parse().unwrap();
            let _ = builtins::exit::main(code);
        } else if line.starts_with("/") {
            let _ = externals::entry::waitfor(&line);
        } else if line.starts_with("& ") {
            let _ = externals::entry::background(&line);
        } else if line.starts_with("exec ") {
            let _ = externals::entry::exec(&line);
        } else if line.starts_with("read") {
            let prompt = line.trim_start_matches("read");
            let prompt = prompt.trim_end_matches("\n");
            if prompt.starts_with('"') && prompt.ends_with('"') {
                let prompt = prompt.trim_start_matches('"') ;
                let prompt = prompt.trim_end_matches('"');
                let _ = builtins::read::read(prompt, &mut lock); 
            } else {
                let _ = builtins::read::read(prompt, &mut lock);
            }
        } else {
            exit(1);
        }
        line.clear();
    }
    return;
}
