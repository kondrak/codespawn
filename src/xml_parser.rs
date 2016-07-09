extern crate xml;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use self::xml::reader::{EventReader, XmlEvent};
use string_gen::keywords::*;
use raw_code::{RawCode, CodeData, generate_raw};

pub fn process_xml(filename: &str) -> RawCode {
    let path = Path::new(&filename);
    let file = match File::open(&path) {
        Err(why) =>  panic!("Couldn't open {} for reading: {}", path.display(), why.description()),
        Ok(file) => file
    };

    let file = BufReader::new(file);
    let parser = EventReader::new(file);
    let mut code_data = Vec::<CodeData>::new();
    let mut config_tags = Vec::<CodeData>::new();
    let mut config_data = Vec::<CodeData>::new();
    let mut depth = 0;

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                let mut attribs = Vec::<(String, String)>::new();
                for a in attributes.iter() {
                    attribs.push((a.name.local_name.clone(), a.value.clone()));
                }
                if name.local_name == CONFIG {
                    config_tags.push((name.local_name, attribs, depth));
                }
                else {
                    code_data.push((name.local_name, attribs, depth));
                }
                depth += 1;
            }
            Ok(XmlEvent::EndElement { .. }) => {
                depth -= 1;
            }
            Err(e) => {
                println!("Error parsing {}: {}", filename, e);
            }
            _ => {}
        }
    }

    // process configs, if found
    for c in config_tags.iter() {
        for a in c.1.iter() {
            if a.0 == FILE {
                let path = Path::new(&a.1);
                let file = match File::open(&path) {
                    Err(why) =>  panic!("Couldn't open {} for reading: {}", path.display(), why.description()),
                    Ok(file) => file
                };
                let file = BufReader::new(file);
                let parser = EventReader::new(file);

                for e in parser {
                    match e {
                        Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                            let mut attribs = Vec::<(String, String)>::new();
                            for a in attributes.iter() {
                                attribs.push((a.name.local_name.clone(), a.value.clone()));
                            }
                            config_data.push((name.local_name, attribs, 0));
                        }
                        Err(e) => {
                            println!("Error parsing {}: {}", a.1, e);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    generate_raw(&code_data, &config_data)
}
