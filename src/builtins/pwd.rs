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

use std::io::Write;

/// █████▄ ██     ██ ████▄  
/// ██▄▄█▀ ██ ▄█▄ ██ ██  ██ 
/// ██      ▀██▀██▀  ████▀  
/// Standard shell `pwd`
#[inline]
pub fn pwd<W: Write>(out: &mut W) -> std::io::Result<()> {
    let pwd = std::env::current_dir()?;
    let _ = write!(out, "{}\n", pwd.display())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(())
}
