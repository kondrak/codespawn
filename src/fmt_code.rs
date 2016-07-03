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
    pub fn new(lang: Lang, cfg: Option<&CodeConfig>, data: &Vec<CodeItem>) -> FormattedCode {
        let mut fmt_code = FormattedCode {
            language: lang,
            elements: Vec::<CodeItem>::new(),
        };

        // todo: populate elements with data using cfg as reference
        fmt_code
    }

    // generate a string with output code
    pub fn to_string(&self) -> String {
        String::from("Generated code\nTODO")
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
}
