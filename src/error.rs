use reqwest;
use serde_json;
use std::io;

#[derive(Debug)]
pub enum BookError {
    IOError(io::Error),
    ReqwestError(reqwest::Error),
    JsonError(serde_json::Error),
}

impl From<io::Error> for BookError {
    fn from(e: io::Error) -> BookError {
        BookError::IOError(e)
    }
}

impl From<reqwest::Error> for BookError {
    fn from(e: reqwest::Error) -> BookError {
        BookError::ReqwestError(e)
    }
}

impl From<serde_json::Error> for BookError {
    fn from(e: serde_json::Error) -> BookError {
        BookError::JsonError(e)
    }
}