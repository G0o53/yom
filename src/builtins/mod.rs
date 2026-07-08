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

//! █████▄ ██  ██ ██ ██    ██████ ██ ███  ██ ▄█████
//! ██▄▄██ ██  ██ ██ ██      ██   ██ ██ ▀▄██ ▀▀▀▄▄▄
//! ██▄▄█▀ ▀████▀ ██ ██████  ██   ██ ██   ██ █████▀
//! This module implements some standard shell builtins for `yom`. These are
//! very high-speed and designed to consume as minimal resources as possible.

pub mod cd;
pub mod echo;
pub mod exit;
pub mod pwd;
pub mod read;
