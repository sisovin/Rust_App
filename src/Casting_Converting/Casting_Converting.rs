fn main() {
    // Convert float to integer
    let float_value = std::f64::consts::PI;
    let int_value = float_value as i32;
    println!("{}", int_value);

    // Convert integer to float
    let int_value = 3;
    let float_value = int_value as f64;
    println!("{}", float_value);

    // Convert string to string (redundant in Rust)
    let string_value = "3.0".to_string();
    println!("{}", string_value);

    // Convert string to integer and add
    let int_value = 100;
    let string_value = "50".parse::<i32>().unwrap();
    println!("{}", int_value + string_value);

    // Convert string to float and add
    let int_value = 100;
    let string_value = "50.99".parse::<f64>().unwrap();
    println!("{}", int_value as f64 + string_value);
}
