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

use std::io::{self};
                             
/// █████▄  ██████ ▄████▄ ████▄  
/// ██▄▄██▄ ██▄▄   ██▄▄██ ██  ██ 
/// ██   ██ ██▄▄▄▄ ██  ██ ████▀  
/// Standard shell `read`
#[inline]
pub fn read(var: &str) -> String {
    let var = var.strip_prefix("$").expect("No $ before the variable");
    let mut var: String = std::env::var(var).unwrap_or_else(|_| String::new());

    io::stdin()
        .read_line(&mut var)
        .expect("Failed to read line from stdin");
    var = var.strip_suffix("\n").unwrap_or(&var).to_string();
    var
}
