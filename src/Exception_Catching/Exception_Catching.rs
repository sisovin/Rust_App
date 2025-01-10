// Exception Catching

fn main() {
    // Handling division by zero
    let result = std::panic::catch_unwind(|| {
        let num = 10;
        let denom = 0;
        if denom != 0 {
            println!("{}", num / denom);
        } else {
            panic!("Division by zero");
        }
    });

    match result {
        Ok(_) => println!("No error during division"),
        Err(_) => println!("Caught panic: Division by zero"),
    }

    // Handling other errors
    let result = std::panic::catch_unwind(|| {
        panic!("Made Up Exception");
    });

    match result {
        Ok(_) => println!("No error during custom panic"),
        Err(_) => println!("Caught panic: Made Up Exception"),
    }
}
