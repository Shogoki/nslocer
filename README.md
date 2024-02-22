# nSLOCer

This is a small command line tool to get the normalized source lines of Code (`nSLOC`), written in Rust. 
Currently it supports the following languages:

- Go
- Rust
- Solidity
- Javascript

It is based on [tokei](https://github.com/XAMPPRocky/tokei), which does most of the heavy lifting.



## Prerequisites

To use `nslocer` with Rust files you need to have rustfmt installed and in your PATH.

## Installation

To install from source using `cargo`.

```bash
cargo install https://github.com/Shogoki/nslocer
```


## Usage

After installation you can simply call `nslocer -l <Language>`.

Example:

```bash
nslocer -l rust
```

Full Help

```bash
nslocer --help
A simple tool to calculate normalized lines of code, utilitizing tokei in the background

Usage: nslocer [OPTIONS] --language <LANGUAGE>

Options:
  -l, --language <LANGUAGE>            [possible values: rust, go, solidity, javascript]
  -p, --path <PATH>                    [default: .]
  -e, --excluded-path <EXCLUDED_PATH>  [default: tests]
  -h, --help                           Print help
  -V, --version                        Print version
```

