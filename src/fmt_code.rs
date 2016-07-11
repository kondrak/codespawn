//! Structures and formatters for language-specific code data.
use std::fmt;
use std::fs::File;
use std::error::Error;
use std::io;
use std::io::Error as IoError;
use std::io::ErrorKind::Other as ReadError;
use std::io::prelude::*;
use std::path::Path;

use raw_code::{CodeItem, CodeConfig, print_code_item};
use string_gen::keywords::*;
use string_gen::{code_to_str};

/// Supported languages.
#[derive(PartialEq, Eq, Hash)]
pub enum Lang {
    Cpp,
    Rust
}

/// Formatted code data representation.
/// Object of this type is already pre-parsed and ready to generate code for the specific
/// language it's been formatted to.
pub struct FormattedCode {
    /// Language name.
    pub language: Lang,
    /// List of code elements, formatted for current language.
    pub elements: Vec<CodeItem>,
    /// Number of tab characters per line (default: 4).
    pub num_tabs: u8,
    /// Tab character to be used (default: space).
    pub tab_char: char,
}

impl FormattedCode {
    #[doc(hidden)]
    pub fn new(lang: Lang, cfg: &Option<&CodeConfig>, data: &Vec<CodeItem>) -> FormattedCode {
        let mut fmt_code = FormattedCode {
            language: lang,
            elements: data.to_vec(),
            num_tabs: 4,
            tab_char: ' '
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
        for (k, v) in config.global_cfg.iter() {
            match k.as_str() {
                NUM_TABS => if !v.is_empty() { self.num_tabs = v.parse::<u8>().unwrap(); },
                TAB_CHAR => if !v.is_empty() { self.tab_char = v.chars().next().unwrap(); },
                _ => {}
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

    /// Saves generated code to file.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate codespawn;
    ///
    /// let raw_code = codespawn::from_xml("examples/sample.xml").unwrap();
    ///
    /// // Create a FormattedCode object for C++ language
    /// let cpp_code = raw_code.to_cpp();
    /// cpp_code.to_file("sample.cpp");
    /// ```      
    pub fn to_file(&self, filename: &str) -> io::Result<()> {
        let code = self.to_string();
        let path = Path::new(&filename);
        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(why) => {
                return Err(IoError::new(ReadError, format!("Failed to open {} for writing: {}",
                                                           path.display(), why.description())));
            }
        };

        try!(file.write_all(code.as_bytes()));
        Ok(())
    }

    /// Generates code and returns it as a `String`
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate codespawn;
    ///
    /// let raw_code = codespawn::from_xml("examples/sample.xml").unwrap();
    ///
    /// // Create a FormattedCode object for C++ language
    /// let cpp_code = raw_code.to_cpp();
    /// println!("Generated C++ code:\n {}", cpp_code.to_string());
    /// ```
    pub fn to_string(&self) -> String {
        code_to_str(self)
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_name = match *self {
            Lang::Cpp => "C/C++", Lang::Rust => "Rust"
        };
        write!(f, "{}", str_name)
    }
}

impl fmt::Display for FormattedCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "Target: {}\n", self.language);
        let _ = write!(f, "*\n");
        for e in self.elements.iter() {
            let mut empty_spaces = Vec::<u8>::new();
            print_code_item(e, f, 0, &mut empty_spaces);
        }
        write!(f, "*\n")
    }
}
