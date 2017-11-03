# RTFM
The program `rtfm` is a computer program that prints out a friendly reminder of places to look for documentation for any unfamiliar computer program you may encounter. If you are not sure how to work with a particular program or third-party piece of software, `rtfm` will point you in the right direction.
It is particularly effective for common questions about a program.

### Installation
Go to the source root in the console. Enter
```
make
make install
```
to build and install `rtfm`.

### Dependencies
The following dependencies are required to build the program.
```
rustc (>= 1.19)
cargo (>= 0.20)
```
In short, a working copy of the Rust compiler and its build tool `cargo`. You can obtain both dependencies from `rust-lang.org`.

### Usage
The program `rtfm` is a terminal program that takes no arguments. Enter
```
rtfm
```
in the terminal or command line and it will produce a pointer to where to find
documentation to solve your computer problems.