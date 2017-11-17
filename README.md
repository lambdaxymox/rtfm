# RTFM -- Read The Friendly Manual

**RTFM** is a computer program that emits a friendly reminder of places to look for documentation for any unfamiliar computer program you may encounter. If you are not sure how to work with a particular program or third-party piece of software, `rtfm` will point you in the right direction. It is particularly effective for common questions about a given computer program. Systems administrators will find this tool useful as part of their standard loadout for new user systems and shell environments.

## Installation

Go to the source root in the console. Enter
```bash
make
make install
```
to build and install `rtfm`.

## Dependencies

The following dependencies are required to build the program.
```
rustc (>= 1.19)
cargo (>= 0.20)
```
In short, a working copy of the Rust compiler and its build tool `cargo`. You can obtain both dependencies from `rust-lang.org`.

## Usage

There are several ways to operate `rtfm`. For example, enter
```bash
rtfm --help
```
in the terminal or command line and it will produce a pointer to where to find
documentation to solve your problems with using `rtfm`. The other option is
```bash
rtfm [program_name]
```
where `[program_name]` is a placeholder for the name of the program you want to learn about. To learn more about `rtfm` itself, simply enter
```bash
rtfm rtfm
```
and you shall be enlightened.