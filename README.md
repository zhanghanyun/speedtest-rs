# Speedtest-rs

No Flash, No Java, No Websocket, No Setup environment, No Docker.

This is a very lightweight Speedtest implemented in [Rust](https://www.rust-lang.org), and only one executable file.

## Installation

You can download the precompiled binary [relaese](https://github.com/zhanghanyun/speedtest-rs/releases) 

##### Build

1. Install [Rust](https://www.rust-lang.org/tools/install) 

2. Cargo build
```shell script
    cargo build --release
```

## Usage
```shell script
speedtest-rs 1.0.0

USAGE:
    speedtest-rs [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --port <port>    Listen port [default: 8088]
```