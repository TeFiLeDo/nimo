# nimo

Network Interface Monitor

## What is this?

This tool helps you to test the reliability and speed of your internet
connection over time.

## Installation

The basic installation is simple:

```
cargo install nimo
```

### Shell Completions

To generate shell completions, run:

```
$ nimo completion <your shell>
```

This will write the completion to stdout, allowing you to redirect it into the
appropriate file. Shell support is determined by `clap`.

### Systemd

The `ping` and `speed-test` subcommands are meant to be executed periodically.
How this happens is up to you. However, systemd service and timer files are
provided to make this easier.

You can simply do this:

```
# cp <path to nimo> /usr/local/bin
$ nimo emit systemd
# cp /tmp/nimo/systemd/* /etc/systemd/system
# systemctl enable --now nimo-ping.timer nimo-speed-test.timer
```

## Usage
```
$ nimo --help
nimo 0.1.0
Adrian Wannenmacher <tfld@tfld.dev>
Network Interface MOnitor

USAGE:
    nimo <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    completion    Generates command completion files for some supported shells
    emit          Emits some provided system configuration files into `/tmp`
    help          Prints this message or the help of the given subcommand(s)
    ping          Tests current internet connectivity utilizing pinging
    speed-test    Tests current internet speed utilizing speedtest.net
```
