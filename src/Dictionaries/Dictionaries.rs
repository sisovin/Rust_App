// Dictionaries

use std::collections::HashMap;

fn main() {
    let mut test_grades: HashMap<&str, &str> = HashMap::new();
    test_grades.insert("Andy", "B+");
    test_grades.insert("Stanley", "C");
    test_grades.insert("Ryan", "A");

    // Update the grade for Andy
    test_grades.insert("Andy", "B-");

    // Print specific elements
    println!("{}", test_grades["Andy"]);
    println!("{}", test_grades["Stanley"]);
    // Note: Rust's HashMap does not support mixed key types directly, so we use string keys here.
}
