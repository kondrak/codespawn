use std::fmt;

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

pub struct RawCode {
    pub elements: Vec<RawCodeItem>
}

impl RawCode {
    pub fn new() -> RawCode {
        RawCode {
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
    fn print_element(&self, e: &RawCodeItem, f: &mut fmt::Formatter, depth: u8) {
        for _ in 0..depth {
            let _ = write!(f, "|  ");
        }

        let _ = write!(f, "|--{}", e.name);
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

        if e.children.len() > 0 {
            for c in e.children.iter() {
                let _ = self.print_element(c, f, depth+1);
            }
            for _ in 0..depth {
                let _ = write!(f, "|  ");
            }

            let _ = write!(f, "|\n");
        }
        else if depth == 0 {
            let _ = write!(f, "|\n");
        }
    }
}

impl fmt::Display for RawCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "\n");
        for e in self.elements.iter() {
            let _ = self.print_element(e, f, 0);
        }
        write!(f, "\n")
    }
}
