fn main() {
    // Define an array with mixed types (not allowed in Rust, so we'll use a vector of strings for demonstration)
    let mut lucky_numbers = ["4", "8", "fifteen", "16", "23", "42.0"];

    // Update the first element
    lucky_numbers[0] = "90";

    // Print specific elements
    println!("{}", lucky_numbers[0]);
    println!("{}", lucky_numbers[1]);
    println!("{}", lucky_numbers[lucky_numbers.len() - 1]);

    println!("\n\n");

    // Print a slice of the array
    println!("{:?}", &lucky_numbers[2..5]);

    println!("\n\n");

    // Print another slice of the array
    println!("{:?}", &lucky_numbers[2..4]);

    println!("\n\n");

    // Print the length of the array
    println!("{}", lucky_numbers.len());
}
