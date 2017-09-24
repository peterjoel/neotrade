extern crate serde_json;
extern crate serde;
extern crate reqwest;
extern crate toml;

#[macro_use]
extern crate serde_derive;

mod model;
mod config;
mod error;

use std::fs::File;
use std::io::Read;
use model::Book;
use error::BookError;
use config::Config;

fn get_order_book(location: &str) -> Result<Book, BookError> {
    let mut resp = reqwest::get(location)?;
    let mut content = String::new();
    resp.read_to_string(&mut content)?;
    let book = serde_json::from_str(&content)?;
    Ok(book)
}

fn main() {
    let file = File::open("config.toml");
    let config = Config::from_file(file.unwrap()).unwrap();
    println!("config = {:?}", config);

    let book = get_order_book(&config.properties.location);
    println!("book = {:?}", book);
}

