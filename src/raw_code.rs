use std::fmt;
use std::collections::HashMap;

pub struct RawCodeItem {
    pub name: String,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<RawCodeItem>
}

impl RawCodeItem {
    pub fn new(item_name: &str, item_attribs: Vec<(String, String)>) -> RawCodeItem {
        RawCodeItem {
            name: String::from(item_name),
            attributes: item_attribs,
            children: Vec::<RawCodeItem>::new()
        }
    }
}

pub struct CodeConfig {
    pub name: String,
    pub type_dict: HashMap<String, String>,
    pub name_dict: HashMap<String, String>
}

impl CodeConfig {
    pub fn new(cfg_name: &str) -> CodeConfig {
        CodeConfig {
            name: String::from(cfg_name),
            type_dict: HashMap::<String, String>::new(),
            name_dict: HashMap::<String, String>::new()
        }
    }
}

pub struct RawCode {
    pub configs: HashMap<String, CodeConfig>,
    pub elements: Vec<RawCodeItem>
}

impl RawCode {
    pub fn new() -> RawCode {
        RawCode {
            configs: HashMap::<String, CodeConfig>::new(),
            elements: Vec::<RawCodeItem>::new()
        }
    }

    // convert raw data to cpp code
    pub fn to_cpp(&self) -> u8 {
        0
    }

    // convert raw data to Rust code
    pub fn to_rust(&self) -> u8 {
        0
    }

    // used by Display trait to print the tree of elements
    fn print_element(&self, e: &RawCodeItem, f: &mut fmt::Formatter, depth: u8, empty_spaces: &mut Vec<u8>) {
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
                self.print_element(&e.children[c], f, depth+1, empty_spaces);
            }

            // reset space directory when topmost child is reached
            if depth == 1 {
                empty_spaces.clear();
            }
        }
    }
}

impl fmt::Display for RawCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "*\n");
        for e in 0..self.elements.len() {
            let mut empty_spaces = Vec::<u8>::new();
            self.print_element(&self.elements[e], f, 0, &mut empty_spaces);
        }
        write!(f, "*\n")
    }
}

impl fmt::Display for CodeConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "Config: {}\n", self.name);
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
