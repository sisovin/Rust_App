// Constructors

struct Book {
    title: String,
    author: String,
}

impl Book {
    // Constructor
    fn new(title: &str, author: &str) -> Book {
        Book {
            title: String::from(title),
            author: String::from(author),
        }
    }

    fn read_book(&self) {
        println!("Reading {} by {}", self.title, self.author);
    }
}

fn main() {
    let book1 = Book::new("Harry Potter", "JK Rowling");

    println!("{}", book1.title);
    book1.read_book();
}
