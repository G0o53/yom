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

use crate::internals::helpers::err_write;
use std::io::Write;
use std::process::{Command, Stdio};

/// ▄█████ ████▄  
/// ██     ██  ██
/// ▀█████ ████▀  
/// Standerd shell `cd`
#[inline]
pub fn cd<E: Write>(dir: &str, hook: &str, stderr: &mut E, err_continue: bool) {
    if hook == "default" || hook == "" {
        let _ = std::env::set_current_dir(&dir);
    } else {
        let output = Command::new(hook)
            .arg("cd")
            .arg(dir)
            .stdin(Stdio::inherit())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to start");

        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout);
            let path = path.trim();

            if !path.is_empty() {
                if let Err(e) = std::env::set_current_dir(path) {
                    err_write("cd hook failed to work", stderr);
                    let _ = write!(stderr, "{e}");
                    if err_continue == false {
                        std::process::exit(1);
                    }
                }
            }
        } else {
            err_write("cd hook failed to give proper directory", stderr);
            if err_continue == false {
                std::process::exit(1);
            } else {
                let _ = write!(stderr, "falling back to builtin cd");
                let _ = std::env::set_current_dir(&dir);
            }
        }
    }
}
