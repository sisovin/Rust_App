// Strings

fn main() {
    let greeting = "Hello";
    // indexes:   01234

    // Length of the string
    println!("{}", greeting.len());

    // Accessing a character by index
    println!("{}", &greeting[0..1]);

    // Checking if a substring is included
    println!("{}", greeting.contains("llo"));
    println!("{}", greeting.contains("z"));

    // Slicing the string
    println!("{}", &greeting[1..4]);
}