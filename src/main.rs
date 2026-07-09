// Copyright (C) 2026 The YOM Contributors
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License Version 2 as
// published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License Version 2 for more details.
//
// You should have received a copy of the GNU General Public License
// Version 2 along with this program. If not, see
// <https://www.gnu.org/licenses/old-licenses/gpl-2.0.html#SEC1>.

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

// ██ ██▄  ▄██ █████▄ ▄████▄ █████▄  ██████ ▄█████
// ██ ██ ▀▀ ██ ██▄▄█▀ ██  ██ ██▄▄██▄   ██   ▀▀▀▄▄▄
// ██ ██    ██ ██     ▀████▀ ██   ██   ██   █████▀

#[cfg(unix)]
use std::fs::File;
use std::io;
use std::io::Write;
use std::io::{BufRead, BufReader};
mod builtins; // adds all the builtins as a module
mod externals; // adds all the functions for externals as a module
mod internals; // adds all dev functions as a module 
use internals::helpers::*; // adds the dev functions like they were written in this file
use std::process::exit;

fn main() {
    let mut args = std::env::args_os();
    args.next();

    let path = match args.next() {
        Some(p) => p,
        None => return,
    };
    let f = File::open(&path).expect("Expected file");

    let mut input_stack: Vec<BufReader<File>> = Vec::new();
    input_stack.push(BufReader::new(f));

    let mut line = String::new();
    let stdout = io::stdout();
    let mut lock = stdout.lock();
    let mut stderr = std::io::stderr();

    let mut error_continue: bool = true;

    let mut cd_hook: String = "".into();
    let mut echo_hook: String = "".into();
    let mut exit_hook: String = "".into();
    let mut pwd_hook: String = "".into();
    let mut read_hook: String = "".into();
    let mut core_hook: String = "".into();

     while !input_stack.is_empty() {
        let bytes = input_stack.last_mut().unwrap().read_line(&mut line).unwrap();

        if bytes == 0 {
            input_stack.pop();
            continue;
        }

        line = line.trim().into();
         eval::<false, _, _>(
            &mut line, 
            &mut input_stack, 
            &mut lock, 
            &mut stderr, 
            &mut error_continue, 
            &mut cd_hook, 
            &mut echo_hook, 
            &mut exit_hook, 
            &mut pwd_hook, 
            &mut read_hook,
            &mut core_hook
        );
     }
    if !core_hook.is_empty() {
        use std::os::unix::net::UnixListener;
        use std::process::{Command, Stdio};
        use std::fs;
        use std::io::Read;
        use std::env;


        let _ = Command::new(&core_hook)
            .arg(&core_hook)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stdin(Stdio::inherit())
            .spawn();

        let socket_path = "/tmp/yom_core.sock";
        let _ = fs::remove_file(socket_path);

        let listener = match UnixListener::bind(socket_path) {
            Ok(listener) => listener,
            Err(_) => {
                err_write("failed to bind to socket_path", &mut stderr);
                let _ = fs::remove_file(socket_path);
                exit(1);
            }
        };
        let (mut stream, _address) = listener.accept()
            .expect("Failed to accept an incoming client connection");
        
        let mut buffer = String::new();
        let mut old_cwd = env::current_dir().expect("Failed to get current working directory");
        
        loop {
            stream.read_to_string(&mut buffer)
                .expect("Failed to read data from the client stream");

            if buffer.starts_with("eval ") {
                let mut core_line: String = buffer.strip_prefix("eval ").unwrap().to_string();
                eval_call(
                    &mut core_line, 
                    &mut input_stack, 
                    &mut lock, 
                    &mut stderr, 
                    &mut error_continue, 
                    &mut cd_hook, 
                    &mut echo_hook, 
                    &mut exit_hook, 
                    &mut pwd_hook,
                    &mut read_hook,
                    &mut core_hook
                );
                let cwd = env::current_dir().expect("Failed to get current working directory");
                if old_cwd != cwd {
                    old_cwd = env::current_dir().expect("Failed to get current working directory");
                    let cwd_string = cwd.to_string_lossy().into_owned();
                    stream.write_all(cwd_string.as_bytes()).expect("Failed to write data to socket");
                
                }
                
            
            }
        }


    } else {
        return;
    }
}

#[inline(never)]
fn eval_call<W: Write, E: Write>(
    mut line: &mut String,
    input_stack: &mut Vec<BufReader<File>>,
    mut lock: &mut W,
    mut stderr: &mut E,
    error_continue: &mut bool,
    cd_hook: &mut String,
    echo_hook: &mut String,
    exit_hook: &mut String,
    pwd_hook: &mut String,
    read_hook: &mut String,
    core_hook: &mut String
) {
    eval::<true, W, E>(
        &mut line, 
        input_stack, 
        &mut lock, 
        &mut stderr, 
        error_continue, 
        cd_hook, 
        echo_hook, 
        exit_hook, 
        pwd_hook,
        read_hook,
        core_hook
        )
}

#[inline(always)]
fn eval<const NO_INLINE: bool, W: Write, E: Write>(
    mut line: &mut String,
    input_stack: &mut Vec<BufReader<File>>,
    mut lock: &mut W,
    mut stderr: &mut E,
    error_continue: &mut bool,
    cd_hook: &mut String,
    echo_hook: &mut String,
    exit_hook: &mut String,
    pwd_hook: &mut String,
    read_hook: &mut String,
    core_hook: &mut String
) {
    if line.starts_with("#") || line == "" {
        // ignores line
        line.clear();
        return;
    } else if line == "then" {
        // ignores line
        line.clear();
        return;
    } else if line == "fi" {
        // ignores line
        line.clear();
        return;
    } else if line.starts_with(". ") || line.starts_with("source ") {
        let mut parts = line.split_whitespace();
        parts.next(); // skip "." or "source"
        if let Some(path) = parts.next() {
            let f = File::open(path).expect("Expected file");
            input_stack.push(BufReader::new(f));
        } else {
            err_write("source requires a path", &mut stderr);
            if !*error_continue {
                exit(1);
            }
        }
        line.clear();
        return;
    } else if line.starts_with("/") || line.starts_with("$") {
        if line.ends_with(" &") {
            externals::entry::background(line.strip_suffix(" &").unwrap()); // lets the program continue (ignoring the zombie)
            line.clear();
            return;
        } else {
            externals::entry::waitfor(line.as_str()); // waits for the program to finish
            line.clear();
            return;
        }
    } else if line.starts_with("set ") {
        let options = line.strip_prefix("set ").unwrap();
        let mut flag = ' ';
        if let Some(c) = options.chars().next() {
            flag = c;
        } else {
            err_write("no flag included in set", &mut stderr);
            if !*error_continue {
                exit(1);
            }
        }
        let options = options.strip_prefix("-").unwrap_or(options);
        let options = options.strip_prefix("+").unwrap_or(options);
        if options.contains("e") {
            if flag == '+' {
                *error_continue = true;
            } else if flag == '-' {
                *error_continue = false;
            }
        } else {
            err_write("you missed (or mispelt) an option in set", &mut stderr);
            if !*error_continue {
                exit(1);
            }
        }
        line.clear();
        return;
    } else if line.starts_with('e') {
        if line.starts_with("exec ") {
            externals::entry::exec(line.as_str()); // changes yom into said program
            err_write("exec failed to overwrite process", &mut stderr);
            if !*error_continue {
                exit(1);
            }
            line.clear();
            return;
        } else if line.starts_with("echo ") {
            // echos the string back to you
            let str = line.strip_prefix("echo ").unwrap(); // removes the string "echo " from the start
            if str.starts_with('"') && str.ends_with('"') {
                // does a bit of trimming for quotes
                let str = str.strip_prefix('"').unwrap();
                let str = str.strip_suffix('"').unwrap();
                builtins::echo::echo(str, &mut lock, &echo_hook, &mut stderr, *error_continue); // calls echo (handing the stdout lock over)
                line.clear();
                return;
            } else {
                builtins::echo::echo(str, &mut lock, &echo_hook, &mut stderr, *error_continue); // calls echo (handing the stdout lock over)
                line.clear();
                return;
            }
        } else if line.starts_with("export ") {
            let tmp = line.strip_prefix("export ").unwrap();

            if let Some((name, data)) = tmp.split_once('=') {
                unsafe {
                    let split = shell_words::split(data).expect("Invalid command syntax");
                    let data: String = split.join(" ");
                    std::env::set_var(name, data);
                }
            }
            line.clear();
            return;
        }
    } else if line.starts_with("cd ") {
        let dir = line.strip_prefix("cd ").unwrap(); // trims the "cd " string from the dir
        if dir.starts_with('"') && dir.ends_with('"') {
            let dir = dir.strip_prefix('"').unwrap(); // trims quotes from start shadowing old dir
            let dir = dir.strip_suffix('"').unwrap(); // trims quotes from start shadowing old dir
            let _ = builtins::cd::cd(dir, &cd_hook, &mut stderr, *error_continue); // calls cd 
            line.clear();
            return;
        } else {
            let _ = builtins::cd::cd(dir, &cd_hook, &mut stderr, *error_continue); // calls cd 
            line.clear();
            return;
        }
    } else if line == "pwd" {
        // echos the current working directory to stdout
        let _ = builtins::pwd::pwd(&mut lock, &mut stderr, &pwd_hook); // calls pwd (handing stdout lock over)
        line.clear();
        return;
    } else if line.starts_with("read") {
        let prompt = line.strip_prefix("read ").unwrap(); // removes the string "read " from the prompt
        if prompt.starts_with('"') && prompt.ends_with('"') {
            let prompt = prompt.strip_prefix('"').unwrap(); // trims quotes
            let prompt = prompt.strip_suffix('"').unwrap(); // trims quotes while shadowing old prompt
            let val = builtins::read::read(prompt, &read_hook); // executes read
            unsafe {
                std::env::set_var(prompt, val);
            }
            line.clear();
            return;
        } else {
            let val = builtins::read::read(prompt, &read_hook); // executes read
            unsafe {
                std::env::set_var(prompt, val);
            }
            line.clear();
            return;
        }
    } else if line == "exit" {
        builtins::exit::exit(0, &exit_hook, &mut stderr, *error_continue); // calls exit on exit code 0
        line.clear();
        return;
    } else if line.starts_with("exit ") {
        let code = line.strip_prefix("exit ").unwrap();
        let val = str_or_int(code, code);
        if val == true {
            let code: i32 = code.parse().unwrap_or(1); // trims "exit " and the newline, then parses it into i32
            builtins::exit::exit(code, &exit_hook, &mut stderr, *error_continue); // executes exit on exit code specified in the file
            line.clear();
            return;
        } else {
            err_write("exit requires an integer", &mut stderr);
            if !*error_continue {
                exit(1);
            }
            line.clear();
            return;
        }
    } else if line.starts_with("if [") {
        let split = shell_words::split(&line).unwrap();
        let left: &str = &split[2];
        let operator: &str = &split[3];
        let right: &str = &split[4];

        let mut indent: i64 = 1;

        // checking if it is a env variable
        if left.starts_with("$") || right.starts_with("$") {
            let tmp = left.strip_prefix("$").unwrap_or(left);
            let left = std::env::var_os(tmp).unwrap_or(tmp.into());
            let left = left.to_str().expect("Not valid UTF-8");

            let tmp = right.strip_prefix("$").unwrap_or(right);
            let right = std::env::var_os(tmp).unwrap_or(tmp.into());
            let right = right.to_str().expect("Not valid UTF-8");

            let val = str_or_int(left, right);

            if val == true {
                let left: i64 = left.parse().unwrap();
                let right: i64 = right.parse().unwrap();
                let val = ncmp(left, right, operator);

                if val == true {
                    line.clear();
                    return;
                } else {
                    line.clear();

                    while input_stack.last_mut().unwrap().read_line(&mut line).unwrap() > 0 {
                        let trimmed = line.trim();
                        if trimmed.starts_with("if [") {
                            indent += 1;
                        }

                        if trimmed == "fi" {
                            line.clear();
                            indent -= 1;
                        }

                        if indent == 0 {
                            line.clear();
                            break;
                        }
                        line.clear();
                    }
                }
            } else {
                let val = strcmp(left, right, operator);

                if val == true {
                    line.clear();
                    return;
                } else {
                    line.clear();

                    while input_stack.last_mut().unwrap().read_line(&mut line).unwrap() > 0 {
                        let trimmed = line.trim();
                        if trimmed.starts_with("if [") {
                            indent += 1;
                        }

                        if trimmed == "fi" {
                            line.clear();
                            indent -= 1;
                        }

                        if indent == 0 {
                            line.clear();
                            break;
                        }
                        line.clear();
                    }
                }
                return;
            }
        } else {
            let val = str_or_int(left, right);

            if val == true {
                let left: i64 = left.parse().unwrap();
                let right: i64 = right.parse().unwrap();
                let val = ncmp(left, right, operator);

                if val == true {
                    line.clear();
                    return;
                } else {
                    line.clear();

                    while input_stack.last_mut().unwrap().read_line(&mut line).unwrap() > 0 {
                        let trimmed = line.trim();
                        if trimmed.starts_with("if [") {
                            indent += 1;
                        }

                        if trimmed == "fi" {
                            line.clear();
                            indent -= 1;
                        }

                        if indent == 0 {
                            line.clear();
                            break;
                        }
                        line.clear();
                    }
                }
                return;
            } else {
                let val = strcmp(left, right, operator);

                if val == true {
                    line.clear();
                    return;
                } else {
                    line.clear();

                    while input_stack.last_mut().unwrap().read_line(&mut line).unwrap() > 0 {
                        let trimmed = line.trim();
                        if trimmed.starts_with("if [") {
                            indent += 1;
                        }

                        if trimmed == "fi" {
                            line.clear();
                            indent -= 1;
                        }

                        if indent == 0 {
                            line.clear();
                            break;
                        }
                        line.clear();
                    }
                }
            }
        }
        line.clear();
        return;
    } else if line.starts_with("hook ") {
        let split = shell_words::split(&line).unwrap();
        let injected: &str = &split[1];
        let path = &split[2..];
        let path: String = path.join("");

        if injected == "cd" {
            *cd_hook = path
        } else if injected == "echo" {
            *echo_hook = path;
        } else if injected == "exit" {
            *exit_hook = path;
        } else if injected == "pwd" {
            *pwd_hook = path;
        } else if injected == "read" {
            *read_hook = path;
        } else if injected == "core" {
            *core_hook = path;
        } else if injected == "builtins" {
            *cd_hook = path.to_owned();
            *echo_hook = path.to_owned();
            *exit_hook = path.to_owned();
            *pwd_hook = path.to_owned();
            *read_hook = path.to_owned();
        } else {
            err_write("hook not valid", stderr);
            if !*error_continue {
                exit(1);
            }
        }

        line.clear();
        return;
    } else {
        err_write("syntax error", &mut stderr);
        if !*error_continue {
            exit(1);
        }
        line.clear();
        return;
    }
}
