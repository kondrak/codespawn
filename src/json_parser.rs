extern crate json;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use raw_code::{RawCode, CodeData, generate_raw};

pub fn process_json(filename: &str) -> RawCode {
    let path = Path::new(&filename);
    let mut file = match File::open(&path) {
        Err(why) =>  panic!("Couldn't open {} for reading: {}", path.display(), why.description()),
        Ok(file) => file
    };

    let mut json_data = String::new();
    let mut code_data   = Vec::<CodeData>::new();
    let mut config_data = Vec::<CodeData>::new();
    let _ = file.read_to_string(&mut json_data);

    let parsed_json = json::parse(json_data.as_str()).unwrap();
    for i in parsed_json.entries() {
        process(&i, &mut code_data, 0);
    }

    generate_raw(&code_data, &config_data)
}

fn process(json: &(&String, &json::JsonValue), code_data: &mut Vec<CodeData>, depth: u8) {
    code_data.push((json.0.clone(), Vec::<(String, String)>::new(), depth));
    let mut attribs = Vec::<(String, String)>::new();
    let idx = code_data.len() - 1;
    
    for i in json.1.entries() {
        if i.1.entries().count() == 0 && i.1.members().count() == 0 {
            attribs.push((i.0.clone(), String::from(i.1.as_str().unwrap())));
        }

        for j in i.1.members() {
            process(&(&i.0, &j), code_data, depth+1);
        }

        if i.1.entries().count() > 0 {
            process(&i, code_data, depth+1);
        }
    }

    // assign collected attributes to element
    code_data[idx].1 = attribs;
}
