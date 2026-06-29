YOM
===

https://github.com/G0o53/yom 
https://crates.io/crates/yom

YOM is the fast, almost scary fast, dash competitor (even though it can only 
execute scripts!). While dash is a nightmare to install, YOM isn't.
    
    echo "This is yom"
    echo "It is FAST"
    echo "It can even match dash in performance."

Installation
============

With man pages
--------------

Clone the repo:
    git clone https://github.com/G0o53/yom
Then:
    make install

That will ask for your password to install the man pages.

For Packaging
-------------

Clone the repo:
    git clone https://github.com/G0o53/yom
Then:
    make

the binary will be at:
    target/release/yom

man pages are at:
    docs/yom.1

With Cargo
----------

Just:
    cargo install yom

Use Cases
---------

YOM is useful for boot-scripts or system-scripts that run a list of tasks and 
use very minimal (and if any) control flow.

Documentation
-------------

Documentation is available at docs/yom.1

License
-------

YOM is licensed under the GPL-v2 license only.
