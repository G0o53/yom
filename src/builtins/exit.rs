// Copyright (C) 2026 G0o53
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

/// ██████ ██  ██ ██ ██████ 
/// ██▄▄    ████  ██   ██   
/// ██▄▄▄▄ ██  ██ ██   ██   
/// Exits with given exit code
#[inline]
pub fn exit(code: i32) {
    std::process::exit(code);
}
