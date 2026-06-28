PREFIX ?= ./dist
build:
	cargo build --release
install:
	cargo install --path .
	sudo mkdir -p /usr/local/share/man/man1/
	sudo cp ./docs/yom.1 /usr/local/share/man/man1/yom.1
uninstall:
	cargo uninstall yom
	sudo rm -f /usr/local/share/man/man1/yom.1
homebrew:
	cargo build --release
	mkdir -p $(PREFIX)/bin
	cp target/release/yom $(PREFIX)/bin/
	mkdir -p $(PREFIX)/share/man/man1
	cp docs/yom.1 $(PREFIX)/share/man/man1/

