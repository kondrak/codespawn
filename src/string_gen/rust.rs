use raw_code::{CodeItem};
use string_gen::keywords::*;

pub fn convert(code_items: &Vec<CodeItem>, num_tabs: u8, tab_char: char) -> String {
    let mut code_str = String::from("");
    for i in code_items.iter() {
        code_str = format!("{}{}", code_str, parse_item(i, 0, num_tabs, tab_char, false));
    }
    code_str
}

fn parse_item(e: &CodeItem, depth: u8, num_tabs: u8, tab_char: char, struct_item: bool) -> String {
    match e.name.as_ref() {
        ENUM   => make_enum(e, depth, num_tabs, tab_char),
        VAR    => make_variable(e, depth, num_tabs, tab_char, struct_item),
        FUNC   => make_function(e, depth, num_tabs, tab_char, struct_item),
        STRUCT => make_struct(e, depth, num_tabs, tab_char),
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
    let mut e_attr = String::from("");
    for a in e.attributes.iter() {
        match a.0.as_ref() {
            NAME  => if !a.1.is_empty() { e_name = format!(" {}", a.1) },
            ATTRIBUTE => if !a.1.is_empty() { e_attr = format!("{}{}\n", start_indent, a.1) },
            _ => {}
        }
    }

    let mut enum_str = format!("{}{}pub enum{}{}", e_attr, start_indent, e_name, " {\n");

    for c in e.children.iter() {
        match c.name.as_ref() {
            VAR => {
                let mut n = String::from("");
                let mut v = String::from("");
                let mut t = String::from("");
                for a in c.attributes.iter() {
                    match a.0.as_ref() {
                        NAME  => n = format!("{}", a.1),
                        VALUE => v = format!("{}", a.1),
                        TYPE  => if !a.1.is_empty() { t = format!(" as {}", a.1) },
                        _ => {}
                    };
                }
                if v.is_empty() {
                    enum_str.push_str(format!("{}{},\n", spaces_str, n).as_str());
                }
                else {
                    enum_str.push_str(format!("{}{} = {}{},\n", spaces_str, n, v, t).as_str());
                }
            },
            _ => panic!("Illegal enum child: {}", c.name),
        }
    }

    enum_str.push_str(format!("{}{}", start_indent, "}\n\n").as_str());
    enum_str
}

fn make_variable(e: &CodeItem, depth: u8, num_tabs: u8, tab_char: char, struct_var: bool) -> String {
    let mut start_indent = String::from("");
    for _ in 0..num_tabs*depth {
        start_indent.push(tab_char);
    }

    let mut n = String::from("");
    let mut t = String::from("");
    let mut v = String::from("");
    let mut q = String::from("const"); // default qualifier for non-struct variable
    for a in e.attributes.iter() {
        match a.0.as_ref() {
            NAME  => n = format!("{}", a.1),
            TYPE  => t = format!("{}", a.1),
            VALUE => if !a.1.is_empty() { v = format!(" = {}", a.1) },
            QUALIFIER => q = format!("{}", a.1),
            _ => {}
        }
    }

    if struct_var {
        format!("{}pub {}: {}{},\n", start_indent, n, t, v)
    }
    else {
        format!("{}pub {} {}: {}{};\n", start_indent, q, n, t, v)
    }
}

fn make_function(e: &CodeItem, depth: u8, num_tabs: u8, tab_char: char, struct_func: bool) -> String {
    let mut start_indent = String::from("");
    for _ in 0..num_tabs*depth {
        start_indent.push(tab_char);
    }

    let mut f_name = String::from("");
    let mut f_type = String::from("");
    let mut f_qual = String::from("");
    for a in e.attributes.iter() {
        match a.0.as_ref() {
            NAME  => f_name = format!("{}", a.1),
            TYPE  => f_type = format!("{}", a.1),
            QUALIFIER => if !a.1.is_empty() { f_qual = format!("{} ", a.1) },
            _ => {}
        }
    }

    let mut func_str = format!("{}pub {}: {}fn(", start_indent, f_name, f_qual);
    let comma = e.children.len() > 1;
    let mut first_arg = true;

    for c in e.children.iter() {
        match c.name.as_ref() {
            VAR => {
                let mut n = String::from("");
                let mut t = String::from("");
                for a in c.attributes.iter() {
                    match a.0.as_ref() {
                        NAME  => if !a.1.is_empty() { n = format!("{}: ", a.1) },
                        TYPE  => t = format!("{}", a.1),
                        _ => {}
                    };
                }
                func_str.push_str(format!("{}{}{}", if comma && !first_arg { ", " } else { "" }, n, t).as_str());
                first_arg = false;
            },
            _ => panic!("Illegal func child: {}", c.name),
        }
    }

    let ret_type = if f_type.is_empty() { f_type } else { format!(" -> {}", f_type) };
    func_str.push_str(format!("){}{}\n", ret_type, if struct_func { "," } else { ";" }).as_str());

    // ignore function pointers
    if !f_name.is_empty() { func_str } else { String::from("") }
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
    let mut s_attr = String::from("");
    for a in e.attributes.iter() {
        match a.0.as_ref() {
            NAME => if !a.1.is_empty() { s_name = format!(" {}", a.1) },
            ATTRIBUTE => if !a.1.is_empty() { s_attr = format!("{}{}\n", start_indent, a.1) },
            _ => {}
        }
    }

    let mut struct_str = format!("{}{}pub struct{}{}", s_attr, start_indent, s_name, " {\n");

    for c in e.children.iter() {
        struct_str.push_str(parse_item(c, depth+1, num_tabs, tab_char, true).as_str());
    }

    struct_str.push_str(format!("{}{}", start_indent, "}\n\n").as_str());
    struct_str
}
