extern crate xml;

use std::fs::File;
use std::io::BufReader;

use self::xml::reader::{EventReader, XmlEvent};
use self::xml::attribute::{OwnedAttribute};
use raw_code::{RawCode, RawCodeItem};

// fetched from XML (element name, attributes, depth in XML structure)
type CodeData = (String, Vec<OwnedAttribute>, u8);

pub fn process_xml(filename: &str) -> RawCode
{
    let file = File::open(filename).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut code_data = Vec::<CodeData>::new();
    let mut depth = 0;

    for e in parser {

        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                code_data.push((name.local_name, attributes, depth));
                depth += 1;
            }
            Ok(XmlEvent::EndElement { .. }) => {
                depth -= 1;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
            _ => {}
        }
    }
    
    generate_raw_data(&code_data)
}

// create RawCode element from pre-parsed XML data
fn generate_raw_data(data: &Vec<CodeData>) -> RawCode {
    let mut raw_code = RawCode::new();

    for i in data.iter() {
        println!("processing {} {}", i.0, i.2);

        if i.2 == 0 {
            raw_code.elements.push(RawCodeItem::new(i.0.clone()));
        }
        else {
            let mut parent = raw_code.elements.last_mut().unwrap();
            process_kids(parent, i.2, i.0.clone());
        }
    }

    for i in raw_code.elements.iter() {
        println!("{} {}", i.name, i.children.len());
    }

    raw_code
}

// recursively process children of each code element
fn process_kids(item: &mut RawCodeItem, depth: u8, name: String) {
    if depth > 1 {
        process_kids(item.children.last_mut().unwrap(), depth-1, name.clone());
    }
    else {
        println!("add child {} to {}", name, item.name);
        item.children.push(RawCodeItem::new(name));
    }
}
