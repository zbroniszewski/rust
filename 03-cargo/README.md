# 03 Cargo

## Table of Contents

- [What is Cargo?](#what-is-cargo)
- [Creating projects with Cargo](#creating-projects-with-cargo)
  - [Limitations](#limitations)
- [Cargo.toml](#cargotoml)
- [Building projects with Cargo](#building-projects-with-cargo)
  - [Building for Release](#building-for-release)
- [Running projects with Cargo](#running-projects-with-cargo)
  - [Running from Release](#running-from-release)
- [Cargo Check](#cargo-check)
- [Notes](#notes)

## What is Cargo?

> Cargo is Rust’s build system and package manager.

## Creating projects with Cargo

`cargo new project_name`

### Limitations

1. Cargo will not create the `.gitignore` file for you if it sees that you are within an existing Git repository.  
   In order to override the behavior, add the `--vcs=git` flag (You will have to delete the additional `.git` folder if necessary)
2. Cargo package names cannot start with digits (I learned this the hard way), ex:
   ```bash
   cargo new 03-cargo

   error: the name `03-cargo` cannot be used as a package name, the name cannot start with a digit
   If you need a package name to not match the directory name, consider using --name flag.
   If you need a binary with the name "03-cargo", use a valid package name, and set the binary name to be different from the package. This can be done by setting the binary filename to `src/bin/03-cargo.rs` or change the name in Cargo.toml with:

   [[bin]]
   name = "03-cargo"
   path = "src/main.rs"
   ```

## Cargo.toml

Cargo.toml is Cargo's version of `package.json` for Node.js.
This file is in the [TOML](https://toml.io/) (Tom's Obvious, Minimal Language) format:
```toml
[package]
name = "cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

## Building projects with Cargo

`cargo build` - Creates an executable under `./target/debug/package-name`

### Building for Release

`cargo build --release` - Creates an executable under `./target/release/package-name`  
Takes longer to compile, but is compiled with optimizations for runtime

## Running projects with Cargo

`cargo run` - Creates an executable under `./target/debug/package-name` AND runs that executable

### Running from Release

`cargo run --release` - Creates an optimized executable under `./target/release/package-name` AND runs that executable

## Cargo Check

`cargo check` - Checks that your program can compile, without producing an executable.  
This is useful because it is faster than `cargo build`, and often the choice while coding and iterating in development.

## Notes

Both `cargo build` and `cargo build --release` seem to have an effect on the byte size of the executable when compared to `rustc` (Ref: [02 Hello World](/02-hello-world#notes)):
- `rustc` - 478168 bytes
- `cargo build` - 482192 bytes
- `cargo build --release` 477424 bytes

`cargo build` appears to create a larger executable than `rustc`, while `cargo build --release` appears to create a smaller one.
