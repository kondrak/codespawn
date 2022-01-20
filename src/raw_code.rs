//! Structures and generators for abstract code data.
use std::fmt;
use std::collections::HashMap;
use fmt_code::{FormattedCode, Lang};
use string_gen::keywords::{NAME, TYPE, VALUE};
use error::{CodeSpawnError, Result};

// (element name, attributes (Vec<name, value>), depth in API file structure)
#[doc(hidden)]
pub type CodeData = (String, Vec<(String, String)>, u8);

/// Representation of a single code element.
#[derive(Clone)]
pub struct CodeItem {
    /// Name of the element.
    pub name: String,
    /// List of element's attributes and properties.
    pub attributes: Vec<(String, String)>,
    /// List of child elements.
    pub children: Vec<CodeItem>
}

impl CodeItem {
    #[doc(hidden)]
    pub fn new(item_name: &str, item_attribs: Vec<(String, String)>) -> CodeItem {
        CodeItem {
            name: String::from(item_name),
            attributes: item_attribs,
            children: Vec::<CodeItem>::new()
        }
    }
}

/// Language-specific configuration map.
pub struct CodeConfig {
    /// Name of the configuration (can be `cpp` or `rust`).
    pub name: String,
    /// Maps abstract type to overriden, language-specific type. Can be empty if no overrides are specified in config.
    pub type_dict: HashMap<String, String>,
    /// Maps abstract name to overriden, language-specific name (variables, functions, attributes etc.). Can be empty if no overrides are specified in config.
    pub name_dict: HashMap<String, String>,
    /// List of global configuration data for given language.
    pub global_cfg: HashMap<String, String>
}

impl CodeConfig {
    #[doc(hidden)]
    pub fn new(cfg_name: &str) -> CodeConfig {
        CodeConfig {
            name: String::from(cfg_name),
            type_dict: HashMap::<String, String>::new(),
            name_dict: HashMap::<String, String>::new(),
            global_cfg: HashMap::<String, String>::new()
        }
    }
}

/// Abstract code data representation.
/// Object of this type can be used to generate desired code.
pub struct RawCode {
    /// Map of language-specific configurations.
    pub configs: HashMap<String, CodeConfig>,
    /// A vector of all code elements.
    pub elements: Vec<CodeItem>,
    supported_langs: HashMap<Lang, String>
}

impl RawCode {
    #[doc(hidden)]
    pub fn new() -> RawCode {
        let mut rc = RawCode {
            configs: HashMap::<String, CodeConfig>::new(),
            elements: Vec::<CodeItem>::new(),
            supported_langs: HashMap::<Lang, String>::new()
        };

        rc.supported_langs.insert(Lang::Cpp, String::from("cpp"));
        rc.supported_langs.insert(Lang::Rust, String::from("rust"));
        
        rc
    }

    /// Converts `RawCode` into C++ `FormattedCode`
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate codespawn;
    ///
    /// let raw_code = codespawn::from_xml("examples/sample.xml").unwrap();
    /// let cpp_code = raw_code.to_cpp();
    /// ```    
    pub fn to_cpp(&self) -> Result<FormattedCode> {
        self.to_lang(Lang::Cpp)
    }

    /// Converts `RawCode` into Rust `FormattedCode`
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate codespawn;
    ///
    /// let raw_code = codespawn::from_xml("examples/sample.xml").unwrap();
    /// let cpp_code = raw_code.to_rust();
    /// ```
    pub fn to_rust(&self) -> Result<FormattedCode> {
        self.to_lang(Lang::Rust)
    }

    fn to_lang(&self, lang: Lang) -> Result<FormattedCode> {
        let lang_idx = some_get!(self.supported_langs.get(&lang));
        FormattedCode::new(lang, &self.configs.get(lang_idx), &self.elements)
    }
}

impl fmt::Display for RawCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "<raw data>\n");
        let _ = write!(f, "*\n");
        for e in self.elements.iter() {
            let mut empty_spaces = Vec::<u8>::new();
            print_code_item(e, f, 0, &mut empty_spaces);
        }
        write!(f, "*\n")
    }
}

impl fmt::Display for CodeConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "Config: {}\n", self.name);
        if self.global_cfg.len() > 0 {
            let _ = write!(f, "Settings:\n");
        }
        for (k, v) in &self.global_cfg {
            let _ = write!(f, "  {} = {}\n", k, match v.as_str() {
                "	" => "<tab>", " " => "<space>", _ => v.as_str() });
        }
        if self.name_dict.len() > 0 {
            let _ = write!(f, "Names:\n");
        }
        for (k, v) in &self.name_dict {
            let _ = write!(f, "  {} = {}\n", k, v);
        }
        if self.type_dict.len() > 0 {
            let _ = write!(f, "Types:\n");
        }
        for (k, v) in &self.type_dict {
            let _ = write!(f, "  {} = {}\n", k, v);
        }
        write!(f, "")
    }
}

// used by Display trait to print the tree of elements
#[doc(hidden)]
pub fn print_code_item(e: &CodeItem, f: &mut fmt::Formatter, depth: u8, empty_spaces: &mut Vec<u8>) {
    // indentation
    for i in 0..depth {
        let mut separator = "|";
        for j in empty_spaces.iter() {
            if i == *j {
                separator = " ";
                break;
            }
        }
        let _ = write!(f, "{}  ", separator);
    }

    let _ = write!(f, "|--{}", e.name);
    // print attributes
    if e.attributes.len() > 0 {
        let _ = write!(f, " [");
        for a in 0..e.attributes.len() {
            if a > 0 && a < e.attributes.len() {
                let _ = write!(f, ", ");
            }
            let _ = write!(f, "{}:\"{}\"", e.attributes[a].0, e.attributes[a].1);
        }
        let _ = write!(f, "]");
    }
    let _ = write!(f, "\n");

    // child processing
    if e.children.len() > 0 {
        for c in 0..e.children.len() {
            if (e.children.len() - c) == 1 {
                empty_spaces.push(depth+1);
            }
            else {
                empty_spaces.sort();
                let idx = empty_spaces.binary_search(&(depth+1));
                match idx { Ok(_) => { empty_spaces.remove(idx.unwrap()); }, _ => {} };
            }
            print_code_item(&e.children[c], f, depth+1, empty_spaces);
        }

        // reset space directory when topmost child is reached
        if depth == 1 {
            empty_spaces.clear();
        }
    }
}

// create RawCode element from pre-parsed data
#[doc(hidden)]
pub fn generate_raw(data: &Vec<CodeData>, config_data: &Vec<CodeData>) -> Result<RawCode> {
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
            match j.0.as_str() {
                NAME  => n = j.1.clone(),
                TYPE  => t = j.1.clone(),
                VALUE => v = j.1.clone(),
                _ => { some_get!(raw_code.configs.get_mut(&i.0)).global_cfg.insert(j.0.clone(), j.1.clone()); }
            }
        }

        if n.len() > 0 {
            some_get!(raw_code.configs.get_mut(&i.0)).name_dict.insert(n.clone(), v.clone());
        }
        if t.len() > 0 {
            some_get!(raw_code.configs.get_mut(&i.0)).type_dict.insert(t.clone(), v.clone());
        }
    }

    for i in data.iter() {
        // if at depth 0, it's a root element, so add it to the main list
        if i.2 == 0 {
            raw_code.elements.push(CodeItem::new(&i.0, i.1.clone()));
        }
        else {
            // recursively process children of a code element
            fn process_kids(item: &mut CodeItem, depth: u8, name: &str, attribs: &Vec<(String, String)>) -> Result<()> {
                if depth > 1 { process_kids(some_get!(item.children.last_mut()), depth-1, name, attribs)?; }
                else         { item.children.push(CodeItem::new(name, attribs.clone())); }

                Ok(())
            }

            let parent = some_get!(raw_code.elements.last_mut());
            process_kids(parent, i.2, &i.0, &i.1)?;
        }
    }

    Ok(raw_code)
}
