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
use std::process::Stdio;

/// ██████ ██  ██ ██████ ▄█████       ██  ██ ▄████▄ ▄████▄ ██ ▄█▀ ▄█████
/// ██▄▄    ████  ██▄▄   ██           ██████ ██  ██ ██  ██ ████   ▀▀▀▄▄▄
/// ██▄▄▄▄ ██  ██ ██▄▄▄▄ ▀█████ ▄▄▄▄▄ ██  ██ ▀████▀ ▀████▀ ██ ▀█▄ █████▀
/// `exec_hooks` executes the given hooks, it requires the `hooks`, `hook_name`
/// which is the argument to give to the hooks, it requires `to_eval` which is what
/// to give to the hooks as another argument, `err_continue` which is a bool that
/// says if it is to continue on errors or not, and finally `stderr` which is a lock
/// onto stderr.
#[inline]
pub fn exec_hooks<E: Write>(
    hooks: &str,
    hook_name: &str,
    to_eval: &str,
    err_continue: &mut bool,
    stderr: &mut E,
) -> i32 {
    let split = shell_words::split(hooks).unwrap();

    for path in split.iter() {
        match std::process::Command::new(path)
            .arg(&hook_name)
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stdin(Stdio::inherit())
            .arg(to_eval)
            .spawn()
        {
            Ok(mut child) => {
                let status = child.wait();
                match status.unwrap().code() {
                    Some(code) => return code,
                    None => return 1,
                }
            }

            Err(_) => {
                err_write("failed to spawn eval hook", stderr);
                if !*err_continue {
                    std::process::exit(1);
                }
                return 1;
            }
        }
    }
    return 1;
}
