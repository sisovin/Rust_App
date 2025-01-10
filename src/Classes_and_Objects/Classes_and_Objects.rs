// Classes and Objects

struct Book {
    title: String,
    author: String,
}

impl Book {
    fn read_book(&self) {
        println!("Reading {} by {}", self.title, self.author);
    }
}

fn main() {
    let book1 = Book {
        title: String::from("Harry Potter"),
        author: String::from("JK Rowling"),
    };

    book1.read_book();
    println!("{}", book1.title);
}
