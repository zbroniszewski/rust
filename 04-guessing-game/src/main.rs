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
