 # lsuart

[English](README.md) | [简体中文](README_zh.md)

## Project Overview

lsuart is a Rust-based serial port list program. The program uses `serialport` and `clap` as its main dependencies, providing a simple command-line interface to access system serial ports.

## Technical Framework

This project uses the following technical frameworks and libraries:

- **Rust**: As the main programming language, providing high performance and memory safety guarantees.
- **serialport**: Library for serial port communication, version 4.5.0.
- **clap**: Library for parsing command-line arguments, version 4.5.17, with `derive` feature enabled.

## Installation

1. Ensure Rust development environment is installed
2. Clone and build the project:

```bash
git clone https://github.com/huanfeng/lsuart.git
cd lsuart
cargo build --release
```

## Usage

### Basic Command

`lsuart [OPTIONS]`

### Available Options

- `--sort <SORT>`: Sorting mode [default: 1]
  - 0: Default sorting
  - 1: Sort by port number

- `-u, --usb-only`: Show only USB serial devices
- `--pci-only`: Show only PCI serial devices
- `--bt-only`: Show only Bluetooth serial devices
- `-v, --verbose`: Show detailed information
- `-h, --help`: Display help information
- `-V, --version`: Display version information

### Examples

1. List all serial devices:
```bash
lsuart
```

2. Show only USB devices sorted by port number:
```bash
lsuart --usb-only --sort 1
```

3. Display detailed device information:
```bash
lsuart --verbose
```

## Output Description

The program displays a list of found serial port devices, including:

- Port name
- Device type (USB/PCI/Bluetooth)
- Additional information in verbose mode (USB devices only):
  - Vendor ID (VID)
  - Product ID (PID)
  - Serial number
  - Manufacturer
  - Product name

## Contributing

Issues and Pull Requests are welcome!

## License

MIT License