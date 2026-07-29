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

/// ██████ ▄█████ ██  ██ ▄████▄
/// ██▄▄   ██     ██████ ██  ██
/// ██▄▄▄▄ ▀█████ ██  ██ ▀████▀      
/// Standard shell `echo`
#[inline]
pub fn echo<W: Write, E: Write>(
    str: &str,
    out: &mut W,
    hook: &str,
    stderr: &mut E,
    err_continue: bool,
) {
    if hook == "default" || hook == "" {
        let _ = write!(out, "{str}\n");
    } else {
        let mut child = Command::new(hook)
            .arg("echo")
            .arg(str)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to start");

        let status = child.wait().unwrap();
        if status.code().unwrap_or(1) > 1 {
            err_write("error hook crashed", stderr);
            if !err_continue {
                std::process::exit(1);
            } else {
                let _ = write!(stderr, "using builtin echo instead of hook echo\n");
                let _ = write!(out, "{str}\n");
            }
        } else {
            ()
        }
    }
}
