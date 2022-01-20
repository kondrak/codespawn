extern crate xml;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use self::xml::reader::{EventReader, XmlEvent};
use string_gen::keywords::*;
use error::{CodeSpawnError, Result};
use raw_code::{RawCode, CodeData, generate_raw};

pub fn process_xml_file(filename: &str) -> Result<RawCode> {
    let path = Path::new(&filename);
    let mut xml_data = String::new();
    let mut file = File::open(&path)?;
    file.read_to_string(&mut xml_data)?;

    process_xml_str(xml_data.as_str())
}

pub fn process_xml_str(xml_str: &str) -> Result<RawCode> {
    let parser = EventReader::from_str(xml_str);
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
            Err(why) => {
                return Err(CodeSpawnError::Xml(why));
            }
            _ => {}
        }
    }

    // process configs, if found
    for c in config_tags.iter() {
        for a in c.1.iter() {
            if a.0 == NAME {
                let path = Path::new(&a.1);
                let file =File::open(&path)?;
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
                        Err(why) => {
                            return Err(CodeSpawnError::Xml(why));
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    generate_raw(&code_data, &config_data)
}
