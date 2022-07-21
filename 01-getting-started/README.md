# 01 Getting Started

## Table of Contents

- [Installation](#installation)
  - [Verifying](#verifying)
  - [Updating](#updating)
  - [Uninstalling](#uninstalling)
- [Offline Documentation](#offline-documentation)

## Installation

`rustup` is a command line tool for managing Rust versions and associated tools.  
Install it with: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`  
This downloads and executes an interactive shell script that will install:
- The Rust compiler (rustc)
- rustup metadata and toolchains
- The Rust package manager (Cargo)

Note: While `rust` and `rustup` are available via brew formula, I've chosen to install via the official shell script.  
`Rust` and `rustup` should not be installed independently, and `rustup` is a dedicated tool for managing `Rust` versions.  
Both installations (`rustup` via brew, or `rustup` via the shell script) can still be automated via something like `ansible`.

### Verifying

- rustup: `rustup --version`  
- Rust compiler: `rustc --version`
- Cargo: `cargo --version`

### Updating

Updating is easy: `rustup update`

### Uninstalling

`rustup self uninstall` will uninstall both:
- rustup
- Rust (rustc)

## Offline Documentation

Offline documentation is provided by rustup:  
- `rustup doc` - opens a browser window to the index of the official Rust documentation.
- `rustup doc --help` - outputs supported CLI flags for offline documentation, ex:  
- `rustup doc --book` - opens a browser window to *"The Rust Programming Language book"* (The book I'm reading now and referencing while taking notes)

> Any time a type or function is provided by the standard library and you’re not sure what it does or how to use it, use the application programming interface (API) documentation to find out:
`rustup doc --std`
