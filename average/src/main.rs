// Project 2: Average

use std::io;

fn main() {
    println!("Enter the numbers");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Couldnt read the numbers.");
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Not a number"))
        .collect();
    let no_of_elements = numbers.iter().count();
    let mut sum = 0;
    for n in &numbers {
        sum = sum + n;
    }

    let avg = sum as f64 / no_of_elements as f64;
    println!("The average of {no_of_elements} numbers is {avg}");
}
