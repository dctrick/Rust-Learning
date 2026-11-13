// Project 1: Add
// Goal: read two numbers from the terminal and print their sum.
// Concepts: let, mut, String::new, read_line, trim, parse, println!
//
// Steps:
// 1. Ask the user for the first number and read it.
// 2. Ask for the second number and read it.
// 3. Convert both to numbers with parse and print the sum.
// 4. Handle bad input later, when you know Result.
use std::io;

fn main() {
    println!("Enter the first number");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n1: i32 = input.trim().parse().expect("Not a number");

    println!("Enter the Second number");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n2: i32 = input.trim().parse().expect("Not a number");

    let sum = n1 + n2;
    println!("Sum of two numbers = {sum}");
}
