extern crate xml;

use std::fs::File;
use std::io::BufReader;

use self::xml::reader::{EventReader, XmlEvent};
use self::xml::attribute::{OwnedAttribute};
use raw_code::{RawCode, RawCodeItem, CodeConfig};

// fetched from XML (element name, attributes, depth in XML structure)
type CodeData = (String, Vec<OwnedAttribute>, u8);

pub fn process_xml(filename: &str) -> RawCode
{
    let file = File::open(filename).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut code_data = Vec::<CodeData>::new();
    let mut config_tags = Vec::<CodeData>::new();
    let mut config_data = Vec::<CodeData>::new();
    let mut depth = 0;

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                if name.local_name == "config" {
                    config_tags.push((name.local_name, attributes, depth));
                }
                else {
                    code_data.push((name.local_name, attributes, depth));
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
    if config_tags.len() > 0 {
        for c in config_tags.iter() {
            for a in c.1.iter() {
                if a.name.local_name == "file" {
                    let file = File::open(&a.value).unwrap();
                    let file = BufReader::new(file);
                    let parser = EventReader::new(file);

                    for e in parser {
                        match e {
                            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                                config_data.push((name.local_name, attributes, 0));
                            }
                            Err(e) => {
                                println!("Error parsing {}: {}", a.value, e);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    generate_raw_data(&code_data, &config_data)
}

// create RawCode element from pre-parsed XML data
fn generate_raw_data(data: &Vec<CodeData>, config_data: &Vec<CodeData>) -> RawCode {
    let mut raw_code = RawCode::new();

    for i in config_data.iter() {
        if !raw_code.configs.contains_key(&i.0) {
            raw_code.configs.insert(i.0.clone(), CodeConfig::new(&i.0));
        }

        let mut n = String::new();
        let mut t = String::new();
        let mut v = String::new();
        // process all config attributes
        for j in i.1.iter() {
            if j.name.local_name == "name" {
                n = j.value.clone();
                continue;
            }
            
            if j.name.local_name == "type" {
                t = j.value.clone();
                continue;
            }

            if j.name.local_name == "value" {
                v = j.value.clone();
                continue;
            }
        }

        if n.len() > 0 {
            assert!(raw_code.configs.get_mut(&i.0).unwrap().name_dict.insert(n.clone(), v.clone()) == None, "Name \"{}\" already defined in config! (duplicate: {}={})", n, n, v);
        }

        if t.len() > 0 {
            assert!(raw_code.configs.get_mut(&i.0).unwrap().type_dict.insert(t.clone(), v.clone()) == None, "Type \"{}\" already defined in config! (duplicate: {}={}", t, t, v);
        }
    }

    for c in raw_code.configs.iter() {
        println!("{}", c.1);
    }

    for i in data.iter() {
        let mut attribs = Vec::<(String, String)>::new();

        for a in i.1.iter() {
            attribs.push((a.name.local_name.clone(), a.value.clone()));
        }

        // if at depth 0, it's a root element, so add it to the main list
        if i.2 == 0 {
            raw_code.elements.push(RawCodeItem::new(&i.0, attribs));
        }
        else {
            let mut parent = raw_code.elements.last_mut().unwrap();
            process_kids(parent, i.2, &i.0, &attribs);
        }
    }

    println!("{}", raw_code);

    raw_code
}

// recursively process children of each code element
fn process_kids(item: &mut RawCodeItem, depth: u8, name: &str, attribs: &Vec<(String, String)>) {
    if depth > 1 {
        process_kids(item.children.last_mut().unwrap(), depth-1, name, attribs);
    }
    else {
        item.children.push(RawCodeItem::new(name, attribs.clone()));
    }
}
