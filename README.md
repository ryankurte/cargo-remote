# Cargo Remote

A WIP remote executor for cargo, allowing tasks to be executed on remote machines.

## Installation

TODO

## Usage



`cargo remote build -- --verbose`

```
cargo-remote 0.1.0
Ryan Kurte <ryankurte@gmail.com>

USAGE:
    cargo-remote [OPTIONS] [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --destination <destination>    Remote host IP for compilation [default: ~/.cargo-remote]
    -e, --env <env>                    Location of cargo env file [default: ~/.cargo/env]
    -r, --remote <host>                Remote host IP for compilation [default: 192.168.1.152]
    -t, --target <target>              Toolchain for remote use [default: x86_64-unknown-linux-gnu]
    -u, --user <user>                  User for remote machine

ARGS:
    <COMMAND>           Command to execute on the remote host [default: build]
    <REMOTE_ARGS>...    Additional arguments for remote command
```
