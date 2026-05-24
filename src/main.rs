// ██    ██  ██████  ███    ███ 
//  ██  ██  ██    ██ ████  ████ 
//   ████   ██    ██ ██ ████ ██ 
//    ██    ██    ██ ██  ██  ██ 
//    ██     ██████  ██      ██ 
//                        ██                           
//                       ░░                            
//  ██████████   ██████   ██ ███████     ██████  ██████
// ░░██░░██░░██ ░░░░░░██ ░██░░██░░░██   ░░██░░█ ██░░░░ 
//  ░██ ░██ ░██  ███████ ░██ ░██  ░██    ░██ ░ ░░█████ 
//  ░██ ░██ ░██ ██░░░░██ ░██ ░██  ░██ ██ ░██    ░░░░░██
//  ███ ░██ ░██░░████████░██ ███  ░██░██░███    ██████ 
// ░░░  ░░  ░░  ░░░░░░░░ ░░ ░░░   ░░ ░░ ░░░    ░░░░░░  


#[cfg(not(unix))]
compile_error!("YOM is available on UNIX-like systems only.");
// Makes sure YOM dosn't compile on windows

// ██╗███╗   ███╗██████╗  ██████╗ ██████╗ ████████╗███████╗
// ██║████╗ ████║██╔══██╗██╔═══██╗██╔══██╗╚══██╔══╝██╔════╝
// ██║██╔████╔██║██████╔╝██║   ██║██████╔╝   ██║   ███████╗
// ██║██║╚██╔╝██║██╔═══╝ ██║   ██║██╔══██╗   ██║   ╚════██║
// ██║██║ ╚═╝ ██║██║     ╚██████╔╝██║  ██║   ██║   ███████║
// ╚═╝╚═╝     ╚═╝╚═╝      ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚══════╝

use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod builtins; // adds all the builtins as a module
mod externals; // adds all the functions for externals as a module
use std::process::exit;

fn main() {
    let mut args = std::env::args_os();
    args.next();
    
    let path = match args.next() {
        Some(p) => p,
        None => return,
    };
    let f = File::open(&path).expect("Expected file");
    
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let stdout = io::stdout();
    let mut lock = stdout.lock();
    while reader.read_line(&mut line).unwrap() > 0 {

// ███████╗██╗   ██╗ █████╗ ██╗     
// ██╔════╝██║   ██║██╔══██╗██║     
// █████╗  ██║   ██║███████║██║     
// ██╔══╝  ╚██╗ ██╔╝██╔══██║██║     
// ███████╗ ╚████╔╝ ██║  ██║███████╗
// ╚══════╝  ╚═══╝  ╚═╝  ╚═╝╚══════╝

        if line.starts_with("#") || line == "\n" || line == "" {
            // ignores line
            line.clear();
            continue;

        } else if line.starts_with("echo ") {
            // echos the string back to you
            let str = line.strip_prefix("echo ").unwrap(); // removes the string "echo " from the start
            let str = str.strip_suffix("\n").unwrap_or(str); // removes the newline from the end and shadows the old str
            if str.starts_with('"') && str.ends_with('"') {
                // does a bit of trimming for quotes
                let str = str.strip_prefix('"').unwrap();
                let str = str.strip_suffix('"').unwrap();
                builtins::echo::echo(str, &mut lock); // calls echo (handing the stdout lock over)
            } else {
                builtins::echo::echo(str, &mut lock); // calls echo (handing the stdout lock over)
            }
        } else if line == "pwd\n" {
            // echos the current working directory to stdout
            let _ = builtins::pwd::pwd(&mut lock); // calls pwd (handing stdout lock over)

        } else if line.starts_with("cd ") {
            let dir = line.strip_prefix("cd ").unwrap(); // trims the "cd " string from the dir
            let dir = dir.strip_suffix("\n").unwrap_or(dir); // trims the newline at the end of cd (shadowing the variable)
            if dir.starts_with('"') && dir.ends_with('"') {
                let dir = dir.strip_prefix('"').unwrap(); // trims quotes from start shadowing old dir
                let dir = dir.strip_suffix('"').unwrap(); // trims quotes from start shadowing old dir
                let _ = builtins::cd::cd(dir); // calls cd 
            } else {
                let _ = builtins::cd::cd(dir); // calls cd 
            }

        } else if line == "exit\n" {
            builtins::exit::exit(0); // calls exit on exit code 0

        } else if line.starts_with("exit ") {
            let code = line.strip_prefix("exit ").unwrap();
            let code: i32 = code.strip_suffix("\n").unwrap_or(code).parse().unwrap_or(1); // trims "exit " and the newline, then parses it into i32
            builtins::exit::exit(code); // executes exit on exit code specified in the file

        } else if line.starts_with("/") {
            externals::entry::waitfor(line.as_str()); // waits for the program to finish

        } else if line.starts_with("& ") {
            let _ = externals::entry::background(line.as_str()); // backgrounds the program (ignoring the resulting zombie)

        } else if line.starts_with("exec ") {
            externals::entry::exec(line.as_str()); // changes yom into said program

        } else if line.starts_with("read") {
            let prompt = line.strip_prefix("read ").unwrap(); // removes the string "read " from the prompt
            let prompt = prompt.strip_suffix("\n").unwrap_or(prompt); // removes the newline from the prompt, shadowing the old prompt 
            if prompt.starts_with('"') && prompt.ends_with('"') {
                let prompt = prompt.strip_prefix('"').unwrap(); // trims quotes
                let prompt = prompt.strip_suffix('"').unwrap(); // trims quotes while shadowing old prompt
                let _ = builtins::read::read(prompt, &mut lock); // executes read
            } else {
                let _ = builtins::read::read(prompt, &mut lock); // executes read
            }

        } else {
            exit(1);
        }
       line.clear(); 
    }
    exit(0);
}
