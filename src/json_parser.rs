extern crate json;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Error as IoError;
use std::io::ErrorKind::Other as ReadError;
use std::io::prelude::*;
use std::path::Path;

use string_gen::keywords::*;
use raw_code::{RawCode, CodeData, generate_raw};

pub fn process_json_file(filename: &str) -> io::Result<RawCode> {
    let path = Path::new(&filename);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => {
            return Err(IoError::new(ReadError, format!("Failed to open {} for reading: {}",
                                                       path.display(), why.description())));
        }
    };

    let mut json_data = String::new();
    try!(file.read_to_string(&mut json_data));

    process_json_str(json_data.as_str())
}

pub fn process_json_str(json_str: &str) -> io::Result<RawCode> {
    let mut code_data   = Vec::<CodeData>::new();
    let mut config_tags = Vec::<CodeData>::new();
    let mut config_data = Vec::<CodeData>::new();
    
    let parsed_json = json::parse(json_str).unwrap();
    for i in parsed_json.entries() {
        if i.0 == CONFIG {
            if i.1.len() == 0 {
                let cfg_file = String::from(i.1.as_str().unwrap());
                config_tags.push((String::from(i.0), vec![(String::from(NAME), cfg_file)], 0));
            }
            else {
                let mut filename_vec = Vec::<(String, String)>::new();
                for e in i.1.entries() {
                    let cfg_file = String::from(e.1.as_str().unwrap());
                    filename_vec.push((String::from(NAME), cfg_file));
                }
                config_tags.push((String::from(i.0), filename_vec, 0));
            }
        }
        else {
            process(&i, &mut code_data, 0);
        }
    }

    // process configs, if found
    let mut json_cfg = String::new();
    for c in config_tags.iter() {
        for a in c.1.iter() {
            if a.0 == NAME {
                let path = Path::new(&a.1);
                let mut file = match File::open(&path) {
                    Ok(file) => file,
                    Err(why) => {
                        return Err(IoError::new(ReadError,format!("Failed to open {} for reading: {}",
                                                                  path.display(), why.description())));
                    }
                };
                json_cfg.clear();
                try!(file.read_to_string(&mut json_cfg));

                let parsed_json = json::parse(json_cfg.as_str()).unwrap();
                for i in parsed_json.entries() {
                    process(&i, &mut config_data, 0);
                }
            }
        }
    }

    Ok(generate_raw(&code_data, &config_data))
}

fn process(json: &(&str, &json::JsonValue), data: &mut Vec<CodeData>, depth: u8) {
    data.push((String::from(json.0), Vec::<(String, String)>::new(), depth));
    let mut attribs = Vec::<(String, String)>::new();
    let idx = data.len() - 1;
    
    for i in json.1.entries() {
        if i.1.entries().count() == 0 && i.1.members().count() == 0 {
            if ATTRIB_ARRAY.contains(&i.0) {
                attribs.push((String::from(i.0), String::from(i.1.as_str().unwrap())));
            }
            else {
                // simplified JSON: unknown keywords will be identified as a type->name pair
                attribs.push((String::from(NAME), String::from(i.1.as_str().unwrap())));
                attribs.push((String::from(TYPE), String::from(i.0)));
            }
        }
        for j in i.1.members() {
            process(&(&i.0, &j), data, depth+1);
        }
        if i.1.entries().count() > 0 {
            process(&i, data, depth+1);
        }
    }

    for j in json.1.members() {
        process(&(&json.0, &j), data, depth);
    }

    // assign collected attributes to element
    data[idx].1 = attribs;

    // if element is only a list of members, remove it from the list
    if json.1.entries().count() == 0 && json.1.members().count() > 0 {
        data.remove(idx);
    }
}
