mod xml_parser;
mod raw_code;
mod fmt_code;

use raw_code::{RawCode};

// read xml, return parsed code data
pub fn from_xml(filename: &str) -> RawCode {
    xml_parser::process_xml(filename)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("Hello world!");
    }
}
