# YOM

![Static Badge](https://img.shields.io/badge/YOM-github.com-blueviolet)
![Static Badge](https://img.shields.io/badge/YOM-crates.io-orange)

> YOM is the fast, almost scary fast, dash competitor (even though it can only 
> execute scripts!). While dash is a nightmare to install, YOM isn't.

```bash
echo "This is yom"
echo "It is FAST"
echo "It can even match dash in performance."
```

<details>
<summary>Installation</summary>

# Installation

Via Homebrew
------------

Tap G0o53/tap using homebrew
```bash
brew tap G0o53/tap
brew trust G0o53/tap/yom
```

Install HEAD (recommended):
```bash
brew install --HEAD yom
```

Install the current stable version:
```bash
brew install yom
```

# With man pages

Clone the repo:
```bash
git clone https://github.com/G0o53/yom
```

Then:
```bash
make install
```

That will then use `sudo` to install the man pages.

For Packaging
-------------

Clone the repo
```bash
git clone https://github.com/G0o53/yom
```

then just use `make`
```bash
make
```

the binary will be at `target/release/yom`
man pages are at `docs/yom.1`

With Cargo
----------

Just:
```bash
cargo install yom
```
</details>

<details>
<summary>Use Cases</summary>

# Use Cases

`YOM` is useful for those who need very minimal resource consumption from a shell, or those who like to customize every single aspect to their exact liking

</details>

<details>
<summary>Documentation</summary>

# Documentation

Depending on how you installed it, `YOM` will either have documentation (man pages) installed on your system, or you will have to find them manually, if you used any of the `brew` methods or the `make install`, then you have man pages to view, to use, simply do
```bash
man yom
```
if you installed it using another method, you will find the file at `docs/yom.1`

</details>

<details>
<summary>License</summary>
<br>
YOM is licensed under the `GPL-v2` license only.

</details>

