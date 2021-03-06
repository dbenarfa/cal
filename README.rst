====================
CLI Calendar command
====================

What the project does
---------------------
This command was inspired by Linux `Cal command <https://linuxhint.com/the-linux-cal-command/>`_ that shows the current month calendar as output (by default), developed using `Rust programming language <https://www.rust-lang.org/>`_.

Why the project is useful
--------------------------
I'm using windows and this command doesn't exist in this os, although it can run in linux/MacOs.
I was learning Rust, so I used it as a learning *small* project.

How users can get started with the project
------------------------------------------
1. Just clone the repo.
2. Run ``cargo run``
3. You can build using ``cargo build --release``, and then copy the executable in an accessible folder eg: c:\\windows\\system32
4. You can use the following options:

Options:
~~~~~~~~
 -d, --day <DAY>        Select a specific day in the calendar [default: 9]
 -h, --help             Print help information
 -m, --month <MONTH>    The month [default: 2]
 -n, --noselect         Deactivate the highlighting of days in the calendar
 -V, --version          Print version information
 -y, --year <YEAR>      The year [default: 2022]
 
Example:
~~~~~~~~
    >>> cal
    February 2022
    Su Mo Tu We Th Fr Sa
        01 02 03 04 05
    06 07 08 09 10 11 12
    13 14 15 16 17 18 19
    20 21 22 23 24 25 26
    27 28

TODO:
~~~~~
- Refactoring
- Write unit tests.
