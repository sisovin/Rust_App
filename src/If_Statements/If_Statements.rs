// If Statements

fn main() {
    let is_student = false;
    let is_smart = false;

    if is_student && is_smart {
        println!("You are a student");
    } else if is_student && !is_smart {
        println!("You are not a smart student");
    } else {
        println!("You are not a student and not smart");
    }

    // >, <, >=, <=, !=, == comparisons
    if 1 > 3 {
        println!("number comparison was true");
    }

    if "a" > "b" {
        println!("string comparison was true");
    }
}
