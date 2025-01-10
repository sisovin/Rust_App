// Numbers

fn main() {
    // Basic Arithmetic: +, -, /, *
    println!("{}", 2 * 3);

    // Exponent
    println!("{}", 2_i32.pow(3));

    // Modulus Op. : returns remainder of 10/3
    println!("{}", 10 % 3);

    // Order of operations
    println!("{}", 1 + 2 * 3);

    // int's and doubles
    println!("{}", 10.0 / 3.0);

    let mut num = 10;

    // +=, -=, /=, *=
    num += 100;
    println!("{}", num);

    let num: f64 = -36.8;

    // Absolute value
    println!("{}", num.abs());

    // Rounding
    println!("{}", num.round());

    // Math class has useful math methods
    println!("{}", 144_f64.sqrt());
    println!("{}", 10_f64.log(10.0));
}
