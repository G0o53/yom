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

use std::io::{self};
use std::process::{Command, Stdio};
use std::io::Write;
use crate::err_write;

/// █████▄  ██████ ▄████▄ ████▄  
/// ██▄▄██▄ ██▄▄   ██▄▄██ ██  ██
/// ██   ██ ██▄▄▄▄ ██  ██ ████▀  
/// Standard shell `read`
#[inline]
pub fn read<E: Write>(var: &str, err_hook: &str, err_continue: bool, hook: &str, stderr: &mut E) -> String {
    if hook == "default" || hook == "" {
        let Some(var) = var.strip_prefix("$") else { 
            err_write("no $ before variable", err_hook, stderr);
            if !err_continue { std::process::exit(1); }
            return String::new();
        };
        let mut var: String = std::env::var(var).unwrap_or_else(|_| String::new());

        io::stdin()
            .read_line(&mut var)
            .expect("Failed to read line from stdin");
        var = var.strip_suffix("\n").unwrap_or(&var).to_string();
        var
    } else {
        let Some(var) = var.strip_prefix("$") else { 
            err_write("no $ before variable", err_hook, stderr);
            if !err_continue { std::process::exit(1); }
            return String::new();
        };
        let output = Command::new(hook)
            .arg("read")
            .arg(var)
            .stdin(Stdio::inherit())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to start");

        if output.status.success() {
            let result_str = String::from_utf8_lossy(&output.stdout);
            result_str.strip_suffix("\n").unwrap_or(&var).to_string()
        } else {
            String::new()
        }
    }
}
