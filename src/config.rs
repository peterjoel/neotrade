use std::fs::File;
use std::io::Read;
use toml;

#[derive(Deserialize, Debug)]
pub struct Properties {
    pub location: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub properties: Properties,
}


impl Config {
    pub fn from_str(data: &str) -> Option<Config> {
        toml::from_str(data).ok()
    }
    
    pub fn from_file(mut file: File) -> Option<Config> {
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .ok()
            .iter()
            .flat_map(|_| {
                // println!("mapping = {:?}", contents);
                Config::from_str(&contents)
                // None
            }).next()
    }
}