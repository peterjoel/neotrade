extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use serde::de::{Deserialize, Deserializer};

#[derive(Debug)]
struct Offer {
    price: f64,
    size: u32,
}

impl Offer {
    fn from_raw(raw: &(String, String)) -> Result<Offer, String> {
        let price = raw.0.parse::<f64>().map_err(|e| e.to_string())?;
        let size = raw.1.parse::<u32>().map_err(|e| e.to_string())?;
        Ok(Offer { price, size })
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct RawBook {
    offers: Vec<(String, String)>,
    timestamp: u64,
}

#[derive(Debug)]
struct Book {
    offers: Vec<Offer>,
    timestamp: u64,
}

impl Book {
    fn from_raw(raw: RawBook) -> Result<Book, String> {
        let offers: Vec<Offer> = raw.offers.iter().map(Offer::from_raw).collect::<Result<_,_>>()?;
        Ok(Book {
            offers: offers,
            timestamp: raw.timestamp
        })
    }
}

fn main() {

    let data = r#"{
            "timestamp":1506166905001,
            "offers":[
                ["0.1234", "120"]
            ]}"#;
    
    let res = Book::from_raw(serde_json::from_str(data).unwrap());
    println!("res = {:?}", res);

}
