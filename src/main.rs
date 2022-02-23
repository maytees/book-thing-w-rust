#[derive(Debug)]
enum BookType {
    NonFiction,
    Fiction,
    HistoricalFiction,
    Textbook,
}

struct Book {
    book_type: BookType,
    name: String,
    author: String,
}

struct Library {
    name: String,
    listof_books: Vec<Book>,
    numof_books: i32,
}

impl Library {
    fn add_book(&mut self, book: Book) {
        self.listof_books.push(book);
        self.numof_books += 1;
    }
}
fn main() {
    println!("\n\n\n");
    let mut barns_n_nobles: Library = Library {
        name: String::from("Barnes N' Nobles"),
        listof_books: Vec::new(),
        numof_books: 0,
    };

    let harry_potter: Book = Book {
        book_type: BookType::Fiction,
        name: String::from("Harry Potter"),
        author: String::from("JK Rowling"),
    };

    barns_n_nobles.add_book(harry_potter);

    for book in barns_n_nobles.listof_books {
        println!(
            "The book \"{}\" was written by \"{}\", and is a {:?} book \n",
            book.name, book.author, book.book_type
        )
    }
}
