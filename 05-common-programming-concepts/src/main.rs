use std::f64;

fn main() {
    immutable_example();
    constants_example();
    shadowing_example();
    integer_overflow_example();
    numeric_operations_example();
    tuples_example();
    arrays_example();
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

fn numeric_operations_example() {
  // addition
  let _sum = 5 + 10;

  // subtraction
  let _difference = 95.5 - 4.3;

  // multiplication
  let _product = 4 * 30;

  // division
  let _quotient = 56.7 / 32.2;
  // results in 0
  let _floored = 2 / 3;

  // remainder
  let _remainder = 43 % 5;
}

fn tuples_example() {
  let tup: (char, i32, f64) = ('A', 45, 45.0);

  // destructuring
  let (a, b, _) = tup;
  // index access
  let c = tup.2;

  println!("{a}, {b}, {:.1}", c);
}

fn arrays_example() {
  // initialize with different values
  let _arr: [u8; 3] = [1, 2, 3];
  // initialize with same values [8, 8, 8]
  let _arr_two = [8; 3];
}
