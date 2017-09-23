extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod model;
use model::Book;

fn main() {
    use std::fs::File;
    use std::io::BufReader;

    let file = File::open("data/data.json").unwrap();
    let reader = BufReader::new(file);
    let book: Book = serde_json::from_reader(reader).unwrap();
    println!("book = {:?}", book);
}

