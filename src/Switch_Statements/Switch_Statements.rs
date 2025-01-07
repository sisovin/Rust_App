// Switch Statements

fn main() {
    let my_grade = "A";

    match my_grade {
        "A" => println!("You Pass"),
        "F" => println!("You fail"),
        _ => println!("Invalid grade"),
    }
}
