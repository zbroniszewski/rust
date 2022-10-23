# 05 Common Programming Concepts <!-- omit in toc -->

## Table of Contents <!-- omit in toc -->

- [Variables & Mutability](#variables--mutability)
- [Constants](#constants)
- [Shadowing](#shadowing)
- [Data Types](#data-types)
  - [Scalar Data Types](#scalar-data-types)
    - [Integers](#integers)
    - [Integer Overflow](#integer-overflow)
    - [Floating-Point Types](#floating-point-types)
    - [Numeric Operations](#numeric-operations)
    - [The Boolean Type](#the-boolean-type)
    - [The Character Type](#the-character-type)
  - [Compound Types](#compound-types)
    - [The Tuple Type](#the-tuple-type)
    - [The Array Type](#the-array-type)
- [Functions](#functions)
  - [Parameters](#parameters)
  - [Statements and Expressions](#statements-and-expressions)
  - [Functions with Return Values](#functions-with-return-values)
- [Comments](#comments)
- [Control Flow](#control-flow)
  - [`if` Expressions](#if-expressions)
  - [Repitition with Loops](#repitition-with-loops)
    - [Repeating Code with `loop`](#repeating-code-with-loop)
    - [Returning Code with `loop`](#returning-code-with-loop)
    - [Loop Labels](#loop-labels)
    - [Conditional Loops with `while`](#conditional-loops-with-while)
    - [Looping through a Collection with `for`](#looping-through-a-collection-with-for)

## Variables & Mutability

Variables in Rust are immutable by default. In this case, once a value is bound to a name, its value cannot be changed. 
The compiler will prevent mutating immutable variables. You can make variables mutable by adding `mut` in front of the variable name.

## Constants

- Constants are also values that are bound to a name.
- Constants are not allowed to change. (always immutable)
- You are not allowed to use the `mut` keyword with Constants.
- You declare Constants using the `const` keyword instead of `let`
- The type of the value *must* be annotated. (even if it can be inferred?)  
- Constants can be declared in any scope, including the global scope.  
- Constants may only be set to a constant expression (computed at compile time), not the result of a value that could only be computed at runtime, e.g.:
  ```rust
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
  ```
- You can refer to [Constant Evaluation](https://doc.rust-lang.org/reference/const_eval.html) for supported expressions.
- Naming convention for constants is to use all uppercase with underscores between words.

## Shadowing

You can declare a new variable with the same name as a previous variable using the `let` keyword.  
Ex:  
```rust
let x = 5;

let x = x + 1;

{
    let x = x * 2;
}
```

> Shadowing is different from marking a variable as mut, because we’ll get a compile-time error if we accidentally
  try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations
  on a value but have the variable be immutable after those transformations have been completed.

- We can change the type of the value but reuse the same name through Shadowing
- We cannot change the type of a value through the use of the `mut` keyword

## Data Types

Every value in Rust is of a certain *data type.*
Rust is a *statically typed* language, which means that it must know the types of all variables at compile time.
The compiler can usually infer the type based on the value and how we use it. In cases where many types are possible,
we must annotate the variable with the type we expect.

### Scalar Data Types

A *scalar* type represents a single value. Rust has four primary scalar types:
- integers
- floating-point numbers
- Booleans
- characters

#### Integers

An integer is a number without a fractional component. They can be *signed* or *unsigned*, which refers to whether or not the value can be negative.
Signed numbers are stored using the [two's compliment](https://en.wikipedia.org/wiki/Two%27s_complement) method.

*Built-in integer types in Rust*

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8	   | u8       |
| 16-bit  | i16	   | u16      |
| 32-bit  | i32	   | u32      |
| 64-bit  | i64	   | u64      |
| 128-bit | i128	 | u128     |
| arch    | isize  | usize    |

The `isize` and `usize` types depend on the architecture of the computer your program is running on, which is denoted
in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.  

You can write integer literals in any of the forms shown in the table below:

*Integer literals*

| Number literals  | Example       |
| ---------------- | ------------- |
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |


- Integer types default to `i32`
- The primary situation in which you’d use `isize` or `usize` is when indexing some sort of collection

#### Integer Overflow

*Integer Overflow* occurs when you try to change an integer variable to a value outside of its range.  
When compiling in debug mode, Rust checks for integer overflow that would cause your program to panic at runtime.  
When compiling in release mode, Rust *does not* include checks for integer overflow. Instead, it performs "two's compliment wrapping*.  

> In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on. The program won’t panic, but the variable
  will have a value that probably isn’t what you were expecting it to have. Relying on integer overflow’s wrapping behavior
  is considered an error.

To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library
for primitive numeric types:
- Wrap in all modes with the `wrapping_*` methods, such as `wrapping_add`
- Return the `None` value if there is overflow with the `checked_*` methods
- Return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods
- Saturate at the value’s minimum or maximum values with `saturating_*` methods

#### Floating-Point Types

Rust has two primitive types for *floating-point* numbers:
- `f32`
- `f64`

The default is `f64` because on modern CPUs, it is roughly the same speed as `f32`, but can be more precise.  
Floating-point numbers are represented using the `IEEE-754` standard. `f32` is a single-precision float, and `f64` has double precision.  
All floating-point types are signed.

#### Numeric Operations

Integer division rounds down to the nearest integer.  
Supported operations:
- addition
- subtraction
- multiplication
- division
- remainder

#### The Boolean Type

The `bool` type has two possible values: `true` or `false` and is one byte in size.

#### The Character Type

Rust's `char` type is the language's most primitive alphabetic type.  
We use single quotes to specify `char` literals, while string literals use double quotes.  
The `char` type is four bytes in size and represents a "Unicode Scalar Value," which includes (but not limited to):
- ASCII
- Accented letters
- Chinese
- Emojis
- Zero-width space

Unicode Scalar Values range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive.

### Compound Types

*Compound Types* can group multiple values into one. Rust has two primitive compound types:
- Tuples
- Arrays

#### The Tuple Type

A Tuple is a way of grouping together values with different data types into a single, compound type.  
Tuples have a fixed length; once declared, they cannot grow or shrink in size.  
There are two methods for accessing values of a Tuple:
- Destructuring
- By index with "dot" operator syntax

A Tuple without any values is called a *unit*: `()`  
Expressions implicitly return the unit value if they don’t return any other value.

#### The Array Type

An *array* is a collection of multiple values. Unlike Tuples, every element of an array must be of the same type.  
Arrays in Rust have a fixed length.  
Arrays are useful when you want your data allocated on the *stack* instead of the *heap*.  
Elements of an array can be accessed by index using the square-bracket syntax: `var[3]`  
Accessing invalid indices of an array can result in a *runtime error*. Rust prevents accessing invalid memory and throws these errors.

## Functions

A function in Rust is defined with the `fn` keyword.  
Functions use *snake case* as the conventional style for function and variable names.  
Rust doesn't care where you define your functions, only that they are in a scope that can be seen by the caller.

### Parameters

In function signatures, you *must* declare the type of each parameter.

### Statements and Expressions

Functions are made up of a series of statements, optionally ending in an expression.  

*Statements* are instructions that perform some action, but do not return any value.  
*Expressions* make an evaluation and return a resulting value.  

In some languages such as C and Ruby, you can write `x = y = 6` in which case both `x` and `y` would be set to the value `6`. This is not possible in Rust due to the distinction between statements and expressions. In the case of Rust, you would be attempting to set `x` to the return value of the statement `y = 6`, but statements do not return values.  

Expressions do not include ending semicolons, i.e.:
```rust
let y = {
    let x = 3;
    x + 1
};
```

Examples of expressions include:
- Numerical operations `5 + 6`
- Literals `6`
- Calling a function
- Calling a macro
- A new scope block with curly braces

### Functions with Return Values

Return types of functions are declared after an arrow: `->`  
The return value of a function is the final expression of the function body. You can also return early from a function using the `return` keyword, but most functions return the last expression implicitly.  

Functions that do not return anything, or end in a statement rather than an expression, return the unit type (an empty tuple).

## Comments

- Single-line comments: `//`
- Multi-line comments: `/*...*/`

## Control Flow

The most common constructs that let you control the flow of execution in Rust are `if` expressions and loops.

### `if` Expressions

The `if` condition *must* be a `bool` type, unlike languages such as Ruby or JavaScript which automatically try to convert to a Boolean.

Because `if` is an expression, we can use it on the right side of a `let` statement:
```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

Values that have the potential to be a result from each arm of an `if` expression *must* be of the same type.

### Repitition with Loops

Rust has three kinds of loops:
- loop
- while
- for

#### Repeating Code with `loop`

`loop {}` will execute a block of code forever until it is explicitly told to stop using the `break` keyword. The `continue` keyword can also be used to immediately continue to the next iteration of the loop.

#### Returning Code with `loop`

One of the uses of a `loop` is to retry an operation you know might fail. You might also need to return the result of that operation out of the loop to the rest of your code.  
To do this, you add the value you want returned following the `break` expression:
```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

#### Loop Labels

If you have nested loops, the `break` and `continue` expressions apply to the innermost loop where they are used.  
You can optionally specify a *loop label* to apply these expressions to higher loops:
```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

#### Conditional Loops with `while`

A `while` loop will iterate while a condition is true. When the condition is no longer true, the program calls `break`, stopping the loop. This behavior is possible using a combination of `loop`, `if`, `else` and `break` but is much more clear and concise with a `while` loop.

#### Looping through a Collection with `for`

While you can loop through elements of a collection using a `while` loop or a `loop`, this usually involves access elements of the collection by their indices, which adds more expensive runtime checks to ensure that index is not out-of-bounds.  
A `for` loop optimizes on these runtime checks, making it the best choice for looping through collections in most cases. 