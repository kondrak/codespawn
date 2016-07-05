use raw_code::{CodeItem};

pub fn convert(code_items: &Vec<CodeItem>) -> String {
    let mut code_str = String::from("");
    for i in code_items.iter() {
        code_str = format!("{}{}", code_str, parse_item(i, 0, 2, ' '));
    }

    code_str
}

fn parse_item(e: &CodeItem, depth: u8, num_tabs: u8, tab_char: char) -> String {
    match e.name.as_ref() {
        "enum" => make_enum(e, depth, num_tabs, tab_char),
        _ => String::from(""),
    }
}

fn make_enum(e: &CodeItem, depth: u8, num_tabs: u8, tab_char: char) -> String {
    let mut start_indent = String::from("");
    let mut spaces_str = String::from("");
    for _ in 0..num_tabs*depth {
        start_indent.push(tab_char);
        spaces_str.push(tab_char);
    }
    for _ in 0..num_tabs {
        spaces_str.push(tab_char);
    }

    let mut enum_str = format!("{}{}", start_indent, "enum {\n");

    for c in e.children.iter() {
        match c.name.as_ref() {
            "var" => {
                let mut n = String::from("");
                let mut v = String::from("");
                for a in c.attributes.iter() {
                    match a.0.as_ref() {
                        "name"  => n = format!("{}", a.1),
                        "value" => v = format!("{}", a.1),
                        _ => {}
                    };
                }
                if v.is_empty() {
                    enum_str.push_str(format!("{}{},\n", spaces_str, n).as_str());
                }
                else {
                    enum_str.push_str(format!("{}{} = {},\n", spaces_str, n, v).as_str());
                }
            },
            _ => panic!("Illegal enum child: {}", c.name),
        }
    }

    enum_str.push_str(format!("{}{}", start_indent, "};\n\n").as_str());
    enum_str
}
