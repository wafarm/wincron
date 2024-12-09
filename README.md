# WinCron

WinCron is a simple program implementing crontab for Windows.

## Usage

Download the binary in the [releases](https://github.com/wafarm/wincron/releases) or (if you want an unreleased version) in [actions](https://github.com/wafarm/wincron/actions).

Put the program into a folder you like and set the program as auto-start if you want it to start with the system.

Run the program and the empty crontab file will be generated in `%LOCALAPPDATA%\wincron\crontab`.

Edit the crontab as you want and the program will auto reload it.

## Compilation

Make sure you have [Rust](https://www.rust-lang.org/) installed on your system.

First, clone the repository:

```shell
git clone https://github.com/wafarm/wincron.git
```

Thanks to the Rust ecosystem, building is very simple:

```shell
cargo build --release
```

Then you can find the binary in `target/release` folder.

## For non-Windows Users

(Why do you need this in the first place?)

The program supports Linux and macOS as well, and nothing really differs from Windows despite that the crontab is located in `~/.config/wincron/crontab`.
