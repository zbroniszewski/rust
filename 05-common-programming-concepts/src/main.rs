fn main() {
    immutable_example();
    constants_example();
    shadowing_example();
    integer_overflow_example();
}

fn immutable_example() {
    // `mut` keyword here is necessary in order to change this value later on
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");
}

fn constants_example() {
    const THREE_HOURS_IN_SECONDS: u16 = 60 * 60 * 3;
    println!("number of seconds in three hours: {}", THREE_HOURS_IN_SECONDS);
}

fn shadowing_example() {
    let x = "example";
    let x = x.len();

    println!("number of bytes: {}", x);
}

fn integer_overflow_example() {
    // compiler error: this arithmetic operation will overflow
    // let mut x: u8 = 255;
    // x = x + 1;
    
    let x: u8 = 255;
    println!("value of x: {}", x);
}
