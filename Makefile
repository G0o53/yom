# Copyright (C) 2026 The YOM Contributors
#
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License Version 2 as 
# published by the Free Software Foundation.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License 
# Version 2 along with this program. If not, see 
# <https://www.gnu.org/licenses/old-licenses/gpl-2.0.html#SEC1>.

PREFIX ?= ./dist
build:
	cargo build --release
install:
	cargo install --path .
	sudo mkdir -p /usr/local/share/man/man1/
	sudo cp ./docs/yom.1 /usr/local/share/man/man1/yom.1
	sudo cp ./docs/yom-hooks.1 /usr/local/share/man/man1/yom-hooks.1
uninstall:
	cargo uninstall yom
	sudo rm -f /usr/local/share/man/man1/yom.1
	sudo rm -f /usr/local/share/man/man1/yom-hooks.1
homebrew:
	cargo build --release
	mkdir -p $(PREFIX)/bin
	cp target/release/yom $(PREFIX)/bin/
	mkdir -p $(PREFIX)/share/man/man1
	cp docs/yom.1 $(PREFIX)/share/man/man1/
	cp docs/yom-hooks.1 $(PREFIX)/share/man/man1/

