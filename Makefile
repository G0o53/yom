build:
	cargo build --release
install:
	cargo install --path .
	sudo mkdir -p /usr/local/share/man/man1/
	sudo cp ./docs/yom.1 /usr/local/share/man/man1/yom.1
uninstall:
	cargo uninstall yom
	sudo rm -f /usr/local/share/man/man1/yom.1
