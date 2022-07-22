# 04 Guessing Game

## Table of Contents

- [The Program](#the-program)
- [Input/Output](#inputoutput)
    - [Print Macros](#print-macros)
- [ParseIntError](#parseinterror)
- [Shadowing](#shadowing)
- [Error Handling](#error-handling)

## The Program

This program generates a random number between 1 - 100 known as the `secret_number` and creates a game loop,
allowing the user to input their guess as many times as they'd like.
The program will inform the user via `stdout` whether their guess is greater than or less than the `secret_number`.
If the user's guess matches the `secret_number`, the user wins and the program exits.  
This program also handles parsing errors on the user's input.

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("Guess the number!");
    println!();

    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("Please input your guess: ");

        // prints to stdout will not be immediate unless they end in a new line character
        // you can control this behavior by manually flushing stdout
        match io::stdout().flush() {
            Ok(()) => (),
            Err(_) => {
                println!("Encountered error flushing stdout");
                break;
            },
        }

        let mut guess = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(num_bytes) => num_bytes,
            Err(_) => {
                println!("Error reading line");
                break;
            }
        };

        println!();

        // trim guess of new line character, then parse to static type (u8)
        // redeclaring a variable is called "shadowing" - often used when converting between types
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            // continue game loop if error parsing to u8
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win!");
                // exit game loop / program
                break;
            }
        }
    }
}
```

## Input/Output

Right away, I discovered something interesting. From `stdin`, Rust was reporting that the byte size of my "a" character was 2 bytes.  
I know that the first 128 characters of ASCII can be represented as 1 byte in UTF8, so this called for further digging.  
My first hunch was of course that pesky new line character, so I found a way to remove it:  
```rust
let guess = guess.trim();
```
The byte length of `guess` (when one of the first 128 characters of ASCII) was now being reported as 1 byte - awesome!

### Print Macros

I also learned that Rust buffers prints to `stdout` until a new line character is found.  
This was relevant to this program because I wanted to print to `stdout` on the same line that I was reading `stdin` from.  
The `println!` macro before `io::stdin().read_line()` will not work, because `println!` ends in a new line character.  
However, the `print!` macro does not add any additional characters.  
But, we do run into the issue where the text you are attempting to print may not be printed immediately due to the buffering mechanism.  
The Rust docs of the `print!` macro detail how to achieve immediate printing with the `print!` macro, which looks something like this:
```rust
io::stdout().flush().unwrap();
```
With this in place, we can print to `stdout` on the same line that we are accepting user input on, ex:
```bash
Enter input here: <user input>
```
This is a lot cleaner than:
```bash
Enter input here:
<user input>
```

## ParseIntError

I encountered a new error while attempting to break my program.  
Since the random number generator is generating a number between 1 and 100 (inclusive of both bounds),
I decided to make our `guess` of this program an unsigned 8-bit integer.  
So, when trying to input numbers greater than 255 (the upper limit for unsigned 8-bit integers), I got the following error:  
```bash
Please input your guess: 256

thread 'main' panicked at 'Please type a number!: ParseIntError { kind: PosOverflow }', src/main.rs:24:42
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
I got a different "kind" of error when attempting negative numbers:  
```bash
Please input your guess: -1

thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/main.rs:24:42
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Shadowing

Redeclaring a variable in Rust is knowing as *"Shadowing"*. It is most often used when converting a value between types.

## Error Handling

There are multiple ways to handle errors on Rust's `Result` enum:
```rust
io::stdin().read_line(&mut guess).expect("Encountered error!");
```
^ This will panic and exit the program with the message passed to `expect` if the `Result` enum returned is of the `Err` variant.
```rust
match io::stdin().read_line(&mut guess) {
    Ok(num_bytes) => num_bytes,
    Err(_) => {
        println!("Error reading line");
        break;
    }
};
```
^ This allows you to handle the possible variants of the `Result` enum, and control for more suited behavior that you might need for your program.  
The `_` is a catch-all here, which catches all `Err` types.
