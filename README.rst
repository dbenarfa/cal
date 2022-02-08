====================
CLI Calendar command
====================

What the project does
---------------------
This command was inspired by Linux `cal command <https://en.wikipedia.org/wiki/Cal_(command)>` that shows the current month calendar as output (by default), developed using `Rust programming language <https://www.rust-lang.org/>`_.

Why the project is useful
--------------------------
I'm using windows and this command doesn't exist in this os, at the same time I was learning Rust, so I used it as a small project.

## How users can get started with the project
1. Just clone the repo.
2. Run `cargo run`
3. You can build `cargo build --release` and then copy the executable in an accessible folder eg: c:\windows\system32
4. You can use the following options:

Options:
~~~~~~~~
 -h, --help               Print help information
 -m, --month <MONTH>      The month [default: current month]
 -s, --select <SELECT>    Select a specific day in the calendar [default: current day]
 -V, --version            Print version information
 -y, --year <YEAR>        The year [default: current year]
