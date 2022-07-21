# 02 Hello World

## Table of Contents

- [The Program](#the-program)
  - [How to Compile](#how-to-compile)
  - [How to Run](#how-to-run)
- [Notes](#notes)

## The Program

```rust
fn main() {
    println!("Hello, world!");
}
```

### How to Compile

`rustc main.rs` (`compile.sh` is included)  
Will save a `main` executable within current working directory.

### How to Run

The executable is available to run directly: `./main`  
Expected output:
```bash
Hello, world!
```


## Notes

There are a lot of interesting findings from this simple program.

1. The `println!` function is actually a macro, which is part of Rust's *"Zero Cost Abstractions."*  
   In this case, `println!` is expanded into further code at compile time. This expansion does not happen at runtime.
2. The size of the hello world executable is almost half a MiB:  
   ```bash
   du -bh ./main
   467K    ./main
   ```
   I've previously read complaints about Rust's executable sizes for simple programs, and I also recall strategies to improve this.
3. While Rust's style is to indent with 4 spaces, the indent length or character (space vs. tab) appears to have no effect on the compiler.
4. `rustc` does not appear to compile to universal binaries (at least by default) on MacOS:
   ```bash
   file main
   main: Mach-O 64-bit executable x86_64
   ```
   This should mean that I *cannot* take my compiled executable on an Intel Macbook and run it on an M1 Macbook. (I'd have to recompile on the M1 Macbook first)  
   Compare this to something like the `echo` command/executable:
   ```bash
   file /bin/echo
   /bin/echo: Mach-O universal binary with 2 architectures: [x86_64:Mach-O 64-bit executable x86_64] [arm64e:Mach-O 64-bit executable arm64e]
   /bin/echo (for architecture x86_64):    Mach-O 64-bit executable x86_64
   /bin/echo (for architecture arm64e):    Mach-O 64-bit executable arm64e
   ```
   As far as I know, this means I should be able to copy my `/bin/echo` to an M1 Macbook (arm64) and expect it to work?

<br>

> Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.

^ I believe I've disproved this in some cases (sort of, when using `rustc` with no options) - I'm probably about to learn how the Rust compiler can compile for multiple targets:

> Just compiling with rustc is fine for simple programs, but as your project grows, you’ll want to manage all the options and make it easy to share your code...
  The Cargo tool... will help you write real-world Rust programs.
