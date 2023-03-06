use json::JsonValue;
use std::fs::File;
use std::io::{BufReader, Read};

/// not in use
pub trait BaseJsonReader {
    fn parse(&self, json: &str) -> JsonValue;
    fn parse_file(&self, path: &str) -> JsonValue {
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        let mut json = String::new();
        reader.read_to_string(&mut json).unwrap();
        self.parse(&json)
    }
}
