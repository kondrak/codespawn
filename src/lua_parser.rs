extern crate hlua;

use std::fs::File;
use self::hlua::functions_read::LuaFunction;
//use std::io::prelude::*;
use std::path::Path;

use string_gen::keywords::{NAME, TYPE, VALUE};
use error::{CodeSpawnError, Result};
use raw_code::{RawCode, CodeData, generate_raw};

pub fn process_lua_file(filename: &str) -> Result<RawCode> {
    let path = Path::new(&filename);
    let file = try!(File::open(&path));
    let mut lua = hlua::Lua::new();
    lua.openlibs();
    try!(lua.execute_from_reader::<(), File>(file));
    
    process(&mut lua)
}

pub fn process_lua_str(lua_str: &str) -> Result<RawCode> {
    let mut lua = hlua::Lua::new();
    lua.openlibs();
    try!(lua.execute::<()>(lua_str));
    
    process(&mut lua)
}

fn process(lua: &mut hlua::Lua) -> Result<RawCode> {
    let mut code_data   = Vec::<CodeData>::new();
    let mut config_data = Vec::<CodeData>::new();

    process_config("rust", lua, &mut config_data);
    process_config("cpp", lua, &mut config_data);

    try!(process_code(lua, &mut code_data));
    generate_raw(&code_data, &config_data)
}


fn process_config(name: &str, lua: &mut hlua::Lua, cfg_data: &mut Vec<CodeData>) {
    let mut cfg_table: Option<hlua::LuaTable<_>> = lua.get(name);

    match cfg_table {
        Some(ref mut t) => {
            cfg_data.push((String::from(name), Vec::<(String, String)>::new(), 0));
            let idx = cfg_data.len() - 1;

            // standard config attributes
            let mut attribs = Vec::<(String, String)>::new();
            for (k, v) in t.iter::<String, String>().filter_map(|e| e) {
                attribs.push((String::from(k), String::from(v)));
            }
            // type overrides
            {
                let mut cfg_types: Option<hlua::LuaTable<_>> = t.get(TYPE);
                match cfg_types {
                    Some(ref mut c) => {
                        for (k, v) in c.iter::<String, String>().filter_map(|e| e) {
                            let mut types = Vec::<(String, String)>::new();
                            types.push((String::from(TYPE), String::from(k)));
                            types.push((String::from(VALUE), String::from(v)));
                            cfg_data.push((String::from(name), types, 0));
                        }
                    }
                    None => {}
                }
            }
            // name overrides
            {
                let mut cfg_names: Option<hlua::LuaTable<_>> = t.get(NAME);
                match cfg_names {
                    Some(ref mut c) => {
                        for (k, v) in c.iter::<String, String>().filter_map(|e| e) {
                            let mut names = Vec::<(String, String)>::new();
                            names.push((String::from(NAME), String::from(k)));
                            names.push((String::from(VALUE), String::from(v)));
                            cfg_data.push((String::from(name), names, 0));
                        }
                    }
                    None => {}
                }
            }

            cfg_data[idx].1 = attribs;
        }
        None => {}
    }
}

fn process_code(lua: &mut hlua::Lua, code_data: &mut Vec<CodeData>) -> Result<()> {
    let mut func: LuaFunction<_> = lua.get("get_code").unwrap();
    let v: String = func.call().unwrap();
    println!("Code string: {}", v);
    Ok(())
}
