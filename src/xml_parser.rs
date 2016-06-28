extern crate xml;

use std::fs::File;
use std::io::BufReader;

use self::xml::reader::{EventReader, XmlEvent};
use raw_code::{RawCode};

pub fn process_xml(filename: &str) -> RawCode
{
    let file = File::open(filename).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);

    let raw_code = RawCode::new();
    
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{}", name);
            }
            Ok(XmlEvent::EndElement { name }) => {
                println!("{}", name);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
            _ => {}
        }
    }

    raw_code
}
