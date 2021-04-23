# nimo

Network Interface Monitor

## What is this?

This tool helps you to test the reliability and speed of your internet
connection over time.

## Prerequisites

You need to install the [speedtest.net cli](https://www.speedtest.net/apps/cli). Run it at least
once yourself, you need to accept their EULA and other stuff.

If you plan to execute speed and ping test automatically, you also need to run the speedtest cli
manually as the user that executes the _nimo_ automatically.

## Installation

The basic installation is simple:

```
cargo install nimo
```

### Shell Completions

To generate shell completions, run:

```
$ nimo emit completion <your shell>
```

This will create a file in `/tmp` containing completion information for your shell. You can copy
that file to wherever it needs to go.

If you prefer to just redirect the completion information to a file, you can use the `--stdout`
option, which true to its name prints the completions to stdout.

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

## Configuration

The configuration is stored in `/etc/nimo.toml` and `~/.config/nimo.toml`. Values from the latter
take precedence. All available configuration options and their default values:

```toml
# the path to the data file
data = "/var/lib/nimo/data"

[ping]
count = 16 # how many pings to send to each target

[ping.targets]
cloudflare = "1.1.1.1"
google = "8.8.8.8"

[speed_test]
enabled = false # if you want to run speed tests, set this to true
```

## Usage

```
$ nimo --help
nimo 0.2.0
Adrian Wannenmacher <tfld@tfld.dev>
Network Interface MOnitor

USAGE:
    nimo <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    emit          Emits some provided system configuration files into `/tmp`
    help          Prints this message or the help of the given subcommand(s)
    ping          Tests current internet connectivity utilizing pinging
    speed-test    Tests current internet speed utilizing speedtest.net
```
