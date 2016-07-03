use std::fs::File;
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use raw_code::{CodeItem, CodeConfig};

#[derive(PartialEq, Eq, Hash)]
pub enum Lang {
    Cpp,
    Rust
}

pub struct FormattedCode {
    pub language: Lang,
    pub elements: Vec<CodeItem>,
}

impl FormattedCode {
    pub fn new(lang: Lang, cfg: &Option<&CodeConfig>, data: &Vec<CodeItem>) -> FormattedCode {
        let mut fmt_code = FormattedCode {
            language: lang,
            elements: data.to_vec(),
        };

        match *cfg {
            Some(config) => {
                // replace types and names using data from config file
                fmt_code.process_config(&config);
            },
            None => {}
        };

        fmt_code
    }

    fn process_config(&mut self, config: &CodeConfig) {
        for (k, v) in config.type_dict.iter() {
            for e in self.elements.iter_mut() {
                FormattedCode::update_element(e, &k, &v);
            }
        }
        for (k, v) in config.name_dict.iter() {
            for e in self.elements.iter_mut() {
                FormattedCode::update_element(e, &k, &v);
            }
        }
    }

    fn update_element(e: &mut CodeItem, key: &String, value: &String) {
        for child in e.children.iter_mut() {
            FormattedCode::update_element(child, key, value);
        }
        for a in e.attributes.iter_mut() {
            if a.1 == *key {
                a.1 = value.clone();
            }
        }
    }

    // save generated code (formatted as string) to file
    pub fn to_file(&self, filename: &str) -> io::Result<()> {
        let code = self.to_string();
        let path = Path::new(&filename);
        let mut file = match File::create(&path) {
            Err(why) => panic!("Couldn't open {} for writing: {}", path.display(), why.description()),
            Ok(file) => file,
        };

        file.write_all(code.as_bytes())
    }

    // generate a string with output code
    pub fn to_string(&self) -> String {
        String::from("Generated code\nTODO")
    }
}
