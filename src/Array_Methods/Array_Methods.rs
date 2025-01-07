// Array Methods

fn main() {
    let mut friends = vec!["Oscar", "Angela", "Kevin"];

    // Print the original list
    println!("{:?}", friends);

    // Reverse the list
    friends.reverse();
    println!("{:?}", friends);

    // Sort the list
    friends.sort();
    println!("{:?}", friends);

    // Check if "Oscar" is in the list
    let contains_oscar = friends.contains(&"Oscar");
    println!("{}", contains_oscar);
}