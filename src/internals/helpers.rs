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

/// ▄█████ ██████ █████▄        ▄████▄ █████▄        ██ ███  ██ ██████ 
/// ▀▀▀▄▄▄   ██   ██▄▄██▄       ██  ██ ██▄▄██▄       ██ ██ ▀▄██   ██   
/// █████▀   ██   ██   ██ ▄▄▄▄▄ ▀████▀ ██   ██ ▄▄▄▄▄ ██ ██   ██   ██   
/// This function checks if an str is an `i64`, if it is, it returns `true` if it is an int, a
/// false if it is a str.
#[inline]
pub fn str_or_int(left: &str, right: &str) -> bool {
        left.parse::<i64>().is_ok() && right.parse::<i64>().is_ok()
}
                               
/// ███  ██ ▄█████ ██▄  ▄██ █████▄ 
/// ██ ▀▄██ ██     ██ ▀▀ ██ ██▄▄█▀ 
/// ██   ██ ▀█████ ██    ██ ██     
/// This function compares 2 numbers, it requires the two numbers, `right` and `left`, and an
/// operator to check for, if it is a true statement, it returns `true`, if it is false, it returns
/// `false`.
#[inline]
pub fn ncmp(left: i64, right: i64, operator: &str) -> bool {
    match operator {
        "==" => {
            if left == right {
                true
            } else {
                false 
            }
        }
        
        "!=" => {
            if left != right {
                true
            } else {
                false 
            }
        }

        "<=" => {
            if left <= right {
                true
            } else {
                false 
            }
        }

        ">=" => {
            if left >= right {
                true 
            } else {
                false 
    
            }
        }

        _ => false
    }
}
                                             
/// ▄█████ ██████ █████▄  ▄█████ ██▄  ▄██ █████▄ 
/// ▀▀▀▄▄▄   ██   ██▄▄██▄ ██     ██ ▀▀ ██ ██▄▄█▀ 
/// █████▀   ██   ██   ██ ▀█████ ██    ██ ██     
/// This function compares two `str`s, using an operator, if the statement given is true, it returns
/// true, if it is `false`, it returns `false`.
#[inline]
pub fn strcmp(left: &str, right: &str, operator: &str) -> bool {
    match operator {
        "==" => {
            if left == right {
                true
            } else {
                false
            }
        }

        "!=" => {
            if left != right {
                true
            } else {
                false
            }
        }

        _ => false
    }
}
