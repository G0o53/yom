build:
	cargo build --release
install:
	cargo install --path .
	sudo cp ./docs/yom.1 /usr/local/share/man/man1/yom.1
uninstall:
	cargo uninstall yom
	rm -f /usr/local/share/man/man1/yom.1
