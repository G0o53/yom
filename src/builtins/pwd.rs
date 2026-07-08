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

/// █████▄ ██     ██ ████▄  
/// ██▄▄█▀ ██ ▄█▄ ██ ██  ██
/// ██      ▀██▀██▀  ████▀  
/// Standard shell `pwd`
pub fn pwd<W: Write, E: Write>(out: &mut W, stderr: &mut E, hook: &str) {
    if hook == "default" || hook == "" {
        if let Ok(pwd) = std::env::current_dir() {
            let _ = write!(out, "{}\n", pwd.display());
        } else {
            err_write("failed to get current directory", stderr);
        }
    } else {
        let mut child = Command::new(hook)
            .arg("pwd")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to start");

        child.wait().unwrap();
    }
}
