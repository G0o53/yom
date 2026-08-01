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

//! ██  ██ ▄████▄ ▄████▄ ██ ▄█▀ ▄█████
//! ██████ ██  ██ ██  ██ ████   ▀▀▀▄▄▄
//! ██  ██ ▀████▀ ▀████▀ ██ ▀█▄ █████▀
//! All the hook-related functions in `yom` are stored here.

use crate::err_write;
use std::io::Write;
use std::process::Command;

/// ██████ ██  ██ ██████ ▄█████       ██  ██ ▄████▄ ▄████▄ ██ ▄█▀
/// ██▄▄    ████  ██▄▄   ██           ██████ ██  ██ ██  ██ ████  
/// ██▄▄▄▄ ██  ██ ██▄▄▄▄ ▀█████ ▄▄▄▄▄ ██  ██ ▀████▀ ▀████▀ ██ ▀█▄
/// `exec_hook` executes the given hook, it requires the path to the hook, 
/// `hook_arg1` which is the first argument to give to the hooks,
/// and it requires `hook_arg2` which is what to give to the hooks as another argument
/// says if it is to continue on errors or not, and finally `stderr` which is a lock
/// onto stderr.
pub fn exec_hook<E: Write>(path: &str, hook_arg1: &str, hook_arg2: &str, stderr: &mut E) -> i32 {
    let mut child = Command::new(path)
        .arg(hook_arg1)
        .arg(hook_arg2)
        .spawn()
        .expect("Hook failed to start");

    let status = child.wait().unwrap();
    let exit_code = if let Some(c) = status.code() {
        c
    } else {
        err_write("process terminated by signal", stderr);
        return 1;
    };
    exit_code
}
