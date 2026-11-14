// Project 3: Area
// Goal: let the user pick a shape (square, circle, triangle) and print its area.
// Concepts: enum, match, functions, f64, std::f64::consts::PI
//
// Steps:
// 1. Define enum Shape with a variant for each shape and its measurements.
// 2. Write fn area(shape: &Shape) -> f64 using match.
// 3. Read the choice and the measurements from the terminal, build a Shape, print the area.

use std::io::{self, stdin};

fn main() {
    println!("Select the type of shape you want to find area of: ");
    println!("1. Square");
    println!("2. Circle");
    println!("3. Triangle");
    println!("4. Rectangle");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input");
    let tp: i32 = input.trim().parse().expect("Not a number");
    let mut area: f64 = 0.0;
    match tp {
        1 => {
            println!("Enter the side of the square( cm )");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read the input");
            let a: f64 = input.trim().parse().expect("Not a number");
            area = a * a * a;
        }
        2 => {
            println!("Enter the radius of the circle ( cm )");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read the input");
            let r: f64 = input.trim().parse().expect("Not a number");
            area = 3.14 * r * r;
        }
        3 => {
            println!("Enter the breadth of the triangle");
            let mut input = String::new();
            stdin()
                .read_line(&mut input)
                .expect("Failed to read the input");
            let b: f64 = input.trim().parse().expect("Not a number");
            println!("Enter the height of the triangle");
            let mut input = String::new();
            stdin()
                .read_line(&mut input)
                .expect("Failed to read the input");
            let h: f64 = input.trim().parse().expect("Not a number");
            area = 0.5 * b * h;
        }

        4 => {
            // l * b
            println!("Enter the length of the rectangle");
            let mut input = String::new();
            stdin()
                .read_line(&mut input)
                .expect("Failed to read the input");
            let l: f64 = input.trim().parse().expect("Not a number");

            println!("Enter the Breadth of the rectangle");
            let mut input = String::new();
            stdin()
                .read_line(&mut input)
                .expect("Failed to read the input");
            let b: f64 = input.trim().parse().expect("Not a number");

            area = l * b;
        }

        _ => {
            println!("Not a valid input")
        }
    }
    let mut tp_words: &str = "None";
    match tp {
        1 => {
            tp_words = "Square";
        }
        2 => {
            tp_words = "Circle";
        }
        3 => {
            tp_words = "Triangle";
        }
        4 => {
            tp_words = "Rectangle";
        }
        _ => {}
    }
    println!("Area of the {tp_words} = {area} cm²");
}
