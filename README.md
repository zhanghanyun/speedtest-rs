# Speedtest-rs

No Flash, No Java, No Websocket, No Setup environment, No Docker.

This is a very lightweight Speedtest implemented in [Rust](https://www.rust-lang.org), and only one executable file.

## Installation

You can download the precompiled binary [release](https://github.com/zhanghanyun/speedtest-rs/releases)

### Build

1. Install [Rust](https://www.rust-lang.org/tools/install) 
```shell script
curl https://sh.rustup.rs -sSf | sh
```
2. Cargo build
```shell script
cargo build --release
```
By default, Rust will statically link all Rust code. However, if you use the standard library, it will dynamically link to the system's libc implementation

If you'd like a 100% static binary, the MUSL libc can be used on Linux

To get support for this target, you use rustup:
```shell script
rustup target add x86_64-unknown-linux-musl
```
To use this new target, pass the --target flag to Cargo:
```shell script
cargo build --target x86_64-unknown-linux-musl
```

## Usage
```shell script
speedtest-rs 1.0.1

USAGE:
    speedtest-rs [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --ip <ip>        Listen ip [default: 0.0.0.0]
    -p, --port <port>    Listen port [default: 8088]
```