// Copyright (C) 2026 The YOM Contributors
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License Version 2 as 
// published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
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

use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod builtins;  // adds all the builtins as a module
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
    
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let stdout = io::stdout();
    let mut lock = stdout.lock();
    while reader.read_line(&mut line).unwrap() > 0 {
        line = line.trim_start().to_string();
                                                        
// ██████ ██  ██ ▄████▄ ██     
// ██▄▄   ██▄▄██ ██▄▄██ ██     
// ██▄▄▄▄  ▀██▀  ██  ██ ██████            

        if line.starts_with("#") || line == "\n" || line == "" {
            // ignores line
            line.clear();
            continue;

        } else if line == "then\n" || line == "then" {
            // ignores line 
            line.clear();
            continue;

        } else if line == "fi\n" || line == "fi" {
            // ignores line
            line.clear();
            continue;

        } else if line.starts_with("/") || line.starts_with("$") {
            if line.ends_with(" &\n") {
                externals::entry::background(line.strip_suffix(" &\n").unwrap()); // lets the program continue (ignoring the zombie)
                line.clear();
                continue;

            } else if line.ends_with(" &") {
                externals::entry::background(line.strip_suffix(" &").unwrap());
                line.clear();
                continue;

            } else {
                externals::entry::waitfor(line.as_str()); // waits for the program to finish
                line.clear();
                continue;
            }

        } else if line.starts_with('e') {
            if line.starts_with("exec ") {
                externals::entry::exec(line.as_str()); // changes yom into said program
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
                    line.clear();
                    continue;
                } else {
                    builtins::echo::echo(str, &mut lock); // calls echo (handing the stdout lock over)
                    line.clear();
                    continue;
                }
            } else if line.starts_with ("export ") {
                let tmp = line.strip_prefix("export ").unwrap();
                
                if let Some((name, data)) = tmp.split_once('=') {
                    unsafe {
                        let data = data.strip_suffix("\n").unwrap_or(data);
                        std::env::set_var(name, data);
                    }
                }
                line.clear();
                continue;
            }

        } else if line.starts_with("cd ") {
            let dir = line.strip_prefix("cd ").unwrap(); // trims the "cd " string from the dir
            let dir = dir.strip_suffix("\n").unwrap_or(dir); // trims the newline at the end of cd (shadowing the variable)
            if dir.starts_with('"') && dir.ends_with('"') {
                let dir = dir.strip_prefix('"').unwrap(); // trims quotes from start shadowing old dir
                let dir = dir.strip_suffix('"').unwrap(); // trims quotes from start shadowing old dir
                let _ = builtins::cd::cd(dir); // calls cd 
                line.clear();
                continue;
            } else {
                let _ = builtins::cd::cd(dir); // calls cd 
                line.clear();
                continue;
            }

        } else if line == "pwd\n" {
            // echos the current working directory to stdout
            let _ = builtins::pwd::pwd(&mut lock); // calls pwd (handing stdout lock over)
            line.clear();
            continue;

        } else if line.starts_with("read") {
            let prompt = line.strip_prefix("read ").unwrap(); // removes the string "read " from the prompt
            let prompt = prompt.strip_suffix("\n").unwrap_or(prompt); // removes the newline from the prompt, shadowing the old prompt 
            if prompt.starts_with('"') && prompt.ends_with('"') {
                let prompt = prompt.strip_prefix('"').unwrap(); // trims quotes
                let prompt = prompt.strip_suffix('"').unwrap(); // trims quotes while shadowing old prompt
                let val = builtins::read::read(prompt); // executes read
                unsafe {
                    std::env::set_var(prompt, val);
                }
                line.clear();
                continue;
            } else {
                let val = builtins::read::read(prompt); // executes read
                unsafe {
                    std::env::set_var(prompt, val);
                }
                line.clear();
                continue;
            }    

        } else if line == "exit\n" {
            builtins::exit::exit(0); // calls exit on exit code 0
            line.clear();
            continue;

        } else if line.starts_with("exit ") {
            let code = line.strip_prefix("exit ").unwrap();
            let code: i32 = code.strip_suffix("\n").unwrap_or(code).parse().unwrap_or(1); // trims "exit " and the newline, then parses it into i32
            builtins::exit::exit(code); // executes exit on exit code specified in the file
            line.clear();
            continue;

        } else if line.starts_with("if ") {
            let split = shell_words::split(&line).unwrap();
            let left: &str = &split[2];
            let operator: &str = &split[3];
            let right: &str = &split[4];

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
                        continue;
                    } else {

                        line.clear();
                
                        while reader.read_line(&mut line).unwrap() > 0 {
                            let trimmed = line.strip_prefix("\n").unwrap_or(&line);
                            if trimmed == "fi" {
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
                        continue;
                    } else {
                        line.clear();
                
                        while reader.read_line(&mut line).unwrap() > 0 {
                            let trimmed = line.strip_prefix("\n").unwrap_or(&line);
                            if trimmed == "fi" {
                                line.clear();
                                break;
                            }
                                line.clear();
                        }
                    }
                }

            } else {
                let val = str_or_int(left, right);
                
                if val == true {
                    let left: i64 = left.parse().unwrap();
                    let right: i64 = right.parse().unwrap();
                    let val = ncmp(left, right, operator);
            
                    if val == true {
                        line.clear();
                        continue;
                    } else {

                        line.clear();
                
                        while reader.read_line(&mut line).unwrap() > 0 {
                            let trimmed = line.strip_prefix("\n").unwrap_or(&line);
                            if trimmed == "fi" {
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
                        continue;
                    } else {
                        line.clear();
                
                        while reader.read_line(&mut line).unwrap() > 0 {
                            let trimmed = line.strip_prefix("\n").unwrap_or(&line);
                            if trimmed == "fi" {
                                line.clear();
                                break;
                            }
                                line.clear();
                        }
                    }
                }
            }
            line.clear();
            continue;

        } else {
            exit(1);
        }
    }
    exit(0);
}
