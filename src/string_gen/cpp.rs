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
        "enum"   => make_enum(e, depth, num_tabs, tab_char),
        "var"    => make_variable(e, depth, num_tabs, tab_char),
        "func"   => make_function(e, depth, num_tabs, tab_char),
        "struct" => make_struct(e, depth, num_tabs, tab_char),
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

    let mut e_name = String::from("");
    for a in e.attributes.iter() {
        match a.0.as_ref() {
            "name"  => e_name = format!(" {}", a.1),
            _ => {}
        }
    }

    let mut enum_str = format!("{}enum{}{}", start_indent, e_name, " {\n");

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

fn make_variable(e: &CodeItem, depth: u8, num_tabs: u8, tab_char: char) -> String {
    let mut start_indent = String::from("");
    for _ in 0..num_tabs*depth {
        start_indent.push(tab_char);
    }

    let mut n = String::from("");
    let mut t = String::from("");
    let mut v = String::from("");
    for a in e.attributes.iter() {
        match a.0.as_ref() {
            "name"  => n = format!("{}", a.1),
            "type"  => t = format!("{}", a.1),
            "value" => v = format!(" = {}", a.1),
            _ => {}
        }
    }

    format!("{}{} {}{};\n", start_indent, t, n, v)
}

fn make_function(e: &CodeItem, depth: u8, num_tabs: u8, tab_char: char) -> String {
    let mut start_indent = String::from("");
    for _ in 0..num_tabs*depth {
        start_indent.push(tab_char);
    }

    let mut f_name = String::from("");
    let mut f_type = String::from("");
    for a in e.attributes.iter() {
        match a.0.as_ref() {
            "name"  => f_name = format!(" {}", a.1),
            "type"  => f_type = format!("{}", a.1),
            _ => {}
        }
    }

    let mut func_str = format!("{}{}{}(", start_indent, f_type, f_name);
    let comma = e.children.len() > 1;
    let mut first_arg = true;
    
    for c in e.children.iter() {
        match c.name.as_ref() {
            "var" => {
                let mut n = String::from("");
                let mut t = String::from("");
                let mut v = String::from("");
                for a in c.attributes.iter() {
                    match a.0.as_ref() {
                        "name"  => n = format!(" {}", a.1),
                        "type"  => t = format!("{}{}", if comma && !first_arg { ", " } else { "" }, a.1),
                        "value" => v = format!(" = {}", a.1),
                        _ => {}
                    };
                }
                func_str.push_str(format!("{}{}{}", t, n, v).as_str());
                first_arg = false;
            },
            _ => panic!("Illegal func child: {}", c.name),
        }
    }

    func_str.push_str(");\n");
    func_str
}

fn make_struct(e: &CodeItem, depth: u8, num_tabs: u8, tab_char: char) -> String {
    let mut start_indent = String::from("");
    let mut spaces_str = String::from("");
    for _ in 0..num_tabs*depth {
        start_indent.push(tab_char);
        spaces_str.push(tab_char);
    }
    for _ in 0..num_tabs {
        spaces_str.push(tab_char);
    }

    let mut s_name = String::from("");
    for a in e.attributes.iter() {
        match a.0.as_ref() {
            "name"  => s_name = format!(" {}", a.1),
            _ => {}
        }
    }    

    let mut struct_str = format!("{}struct{}{}", start_indent, s_name, " {\n");

    for c in e.children.iter() {
        struct_str.push_str(parse_item(c, depth+1, num_tabs, tab_char).as_str());
    }

    struct_str.push_str(format!("{}{}", start_indent, "};\n\n").as_str());
    struct_str
}
