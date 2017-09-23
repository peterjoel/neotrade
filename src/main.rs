extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use std::fmt;
use serde::de::{self, Deserialize, Deserializer, SeqAccess, MapAccess, Visitor};

#[derive(Debug)]
struct Offer {
    price: f64,
    size: u32,
}

impl Offer {
    fn new(price: f64, size: u32) -> Offer {
        Offer { price: price, size: size }
    }
}

#[derive(Debug)]
struct Book {
    offers: Vec<Offer>,
    timestamp: u64,
}

impl Book {
    fn new(timestamp: u64, offers: Vec<Offer>) -> Book {
        Book { timestamp: timestamp, offers: offers }
    }
}

struct OfferVisitor;
struct BookVisitor;

impl<'de> Visitor<'de> for OfferVisitor {
    type Value = Offer;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an offer")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Offer, V::Error>
        where V: SeqAccess<'de>
    {
        let price: String = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let size: String = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        Ok(Offer::new(price.parse().unwrap(), size.parse().unwrap()))
    }
}

impl<'de> Deserialize<'de> for Offer {
    fn deserialize<D>(deserializer: D) -> Result<Offer, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_seq(OfferVisitor)
    }
}


impl<'de> Visitor<'de> for BookVisitor {
    type Value = Book;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an order book of offers")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Book, V::Error>
        where V: MapAccess<'de>
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum BookField { Timestamp, Offers }

        let mut timestamp = None;
        let mut offers = None;

        while let Some(key) = map.next_key()? {
            match key {
                BookField::Timestamp => {
                    if timestamp.is_some() {
                        return Err(de::Error::duplicate_field("timestamp"));
                    }
                    timestamp = Some(map.next_value()?);
                }
                BookField::Offers => {
                    if offers.is_some() {
                        return Err(de::Error::duplicate_field("offers"));
                    }
                    offers = Some(map.next_value()?);
                }
            }
        }
        if timestamp.is_none() {
            return Err(de::Error::missing_field("timestamp"));
        }
        if offers.is_none() {
            return Err(de::Error::missing_field("offers"));
        }
        Ok(Book::new(timestamp.unwrap(), offers.unwrap()))
    }
}

impl<'de> Deserialize<'de> for Book {
    fn deserialize<D>(deserializer: D) -> Result<Book, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_map(BookVisitor)
    }
}

fn main() {

    let data = r#"{
            "timestamp":1506166905001,
            "offers":[
                ["0.1234", "120"], ["0.1253", "126"]
            ]}"#;
    
    let book: Book = serde_json::from_str(data).unwrap();
    println!("book = {:?}", book);
}
