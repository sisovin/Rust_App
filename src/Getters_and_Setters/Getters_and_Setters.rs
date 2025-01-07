// Getters and Setters

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

    // Setter for title
    fn set_title(&mut self, title: &str) {
        println!("Set");
        self.title = String::from(title);
    }

    // Getter for title
    fn get_title(&self) -> &str {
        println!("Get");
        &self.title
    }

    // Getter for author
    fn get_author(&self) -> &str {
        &self.author
    }
}

fn main() {
    let mut book1 = Book::new("Harry Potter", "JK Rowling");

    // Using the setter
    book1.set_title("Harry Potter and the Chamber of Secrets");

    // Using the getter
    println!("{}", book1.get_title());
    println!("{}", book1.get_author());
}
