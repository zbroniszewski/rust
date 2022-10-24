# 06 Ownership <!-- omit in toc -->

## Table of Contents <!-- omit in toc -->

- [What is Ownership?](#what-is-ownership)
- [The Stack and the Heap](#the-stack-and-the-heap)
- [Ownership Rules](#ownership-rules)
- [Variable Scope](#variable-scope)
- [The `String` Type](#the-string-type)

## What is Ownership?
*Ownership* is a set of rules that govern how a Rust program manages memory.  
Some languages have garbage collection that regularly looks for no-longer used memory. Other languages require the programmer to explicitly allocate and free memory. Rust uses a third approach called *Ownership* with a set of rules that the compiler checks. If any of the rules are violated, the program won't compile.

## The Stack and the Heap

Whether a value is on the stack or the heap affects how the language behaves and why you have to make certain decisions.  

The stack stores values in the order that it gets them, but removes them in the opposite order. This is called *last in, first out*. All data stored on the stack must have a known, fixed size. Data with an unknown size or a size that might change must be stored on the heap instead. Adding data is called *pushing onto the stack* and removing data is called *popping off the stack*.

When you put data on the heap, you request a certain amount of space. The memory allocator finds a space big enough, marks it as being in use, and returns a *pointer*: an address of that location. This process is called *allocating on the heap* or *allocating*. The pointer to the heap is a known, fixed size that you can store on the stack, but when you want the actual data, you must follow the pointer.

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack.  
Allocating on the heap requires more work, because the allocator must find a space big enough, then perform bookkeeping to prepare for the next allocation.

Accessing data in the heap is slower than the stack because you have to follow a pointer to get there. Contemporary processors are faster if they jump around less in memory. (Cache?)

When your code calls a function, the values passed into the function and the function's local variables get pushed onto the stack. When the function is over, those values get popped off the stack. These values can include pointers to data on the heap.

Ownership addresses the problems of keeping track of what parts of code are using what data on the heap, minimizing duplicate data on the heap, and cleaning up unused data on the heap.

## Ownership Rules

- Each value in Rust has an *owner*
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

## Variable Scope

- When a variable comes into scope, it is valid.
- It remains valid until it goes out of scope.

## The `String` Type



