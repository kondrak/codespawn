mod xml_parser;
mod json_parser;
mod raw_code;
mod fmt_code;
mod string_gen;

use std::io;
use raw_code::{RawCode};

// read xml, return parsed code data
pub fn from_xml(filename: &str) -> io::Result<RawCode> {
    xml_parser::process_xml(filename)
}

pub fn from_json(filename: &str) -> io::Result<RawCode> {
    json_parser::process_json(filename)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("Hello world!");
    }
}
