// User Input

use std::io::{self, Write};

fn main() {
    // Prompt for and read the user's name
    print!("Enter your name: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim(); // Remove any trailing newline characters

    println!("Hello {}, how are you?", name);

    // Prompt for and read the first number
    print!("Enter first num: ");
    io::stdout().flush().unwrap();
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");
    let num1: f64 = num1.trim().parse().expect("Please type a number!");

    // Prompt for and read the second number
    print!("Enter second num: ");
    io::stdout().flush().unwrap();
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");
    let num2: f64 = num2.trim().parse().expect("Please type a number!");

    // Print the sum of the two numbers
    println!("The sum is: {}", num1 + num2);
}
