// For Loops

fn main() {
    // For loop with range
    for index in 0..5 {
        println!("{}", index);
    }

    // Equivalent to 5.times do |index| in Ruby
    for index in 0..5 {
        println!("{}", index);
    }

    // Iterating over an array
    let lucky_nums = [4, 8, 15, 16, 23, 42];
    for lucky_num in lucky_nums.iter() {
        println!("{}", lucky_num);
    }

    // Using for_each method
    lucky_nums.iter().for_each(|&lucky_num| {
        println!("{}", lucky_num);
    });
}
