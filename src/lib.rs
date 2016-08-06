//! [Codespawn](https://github.com/kondrak/codespawn) is a basic C++ and Rust code generator.
//! Desired API can be defined using either JSON or XML and the crate supports both reading
//! from a file or a string.
//!
//! Currently it's possible to generate enums, structs, functions, function pointers, variables
//! and bitflags with all applicable attributes and properties.
//!
//! See [example XML](https://github.com/kondrak/codespawn/blob/master/examples/sample.xml) for instructions
//! on how to construct the API definition.
//!
//!# Quick Start
//!
//!```
//!extern crate codespawn;
//!
//!fn main()
//!{
//!    // generate from XML definition
//!    let raw_code = codespawn::from_xml("examples/sample.xml").unwrap();
//!    // generate from JSON definition
//!    //let raw_code = codespawn::from_json("examples/sample.json").unwrap();
//!
//!    // generate code, store as String
//!    let cpp_code  = raw_code.to_cpp().unwrap().to_string();
//!    let rust_code = raw_code.to_rust().unwrap().to_string();
//!
//!    // generate C++ and save directly to file
//!    raw_code.to_cpp().unwrap().to_file("examples/sample.cpp");
//!    // generate Rust and save directly to file
//!    //raw_code.to_rust().unwrap().to_file("examples/sample.rs");
//!}
//!```
#[macro_use]
pub mod error;
pub mod raw_code;
pub mod fmt_code;
mod xml_parser;
mod json_parser;
mod lua_parser;
mod string_gen;

use error::Result;
use raw_code::{RawCode};

/// Reads XML data from file and compiles it into `RawCode`
///
/// # Examples
///
/// ```
/// extern crate codespawn;
///
/// let raw_code = codespawn::from_xml("examples/sample.xml").unwrap();
/// ```
pub fn from_xml(filename: &str) -> Result<RawCode> {
    xml_parser::process_xml_file(filename)
}

/// Reads XML data from a `&str` and compiles it into `RawCode`
///
/// # Examples
///
/// ```
/// extern crate codespawn;
///
/// let raw_code = codespawn::from_xml_str("<enum name=\"Foo\"><var name=\"EnumVal1\" type=\"int\" /></enum>").unwrap();
/// ```
pub fn from_xml_str(xml: &str) -> Result<RawCode> {
    xml_parser::process_xml_str(xml)
}

/// Reads JSON data from file and compiles it into `RawCode`
///
/// # Examples
///
/// ```
/// extern crate codespawn;
///
/// let raw_code = codespawn::from_json("examples/sample.json").unwrap();
/// ```
pub fn from_json(filename: &str) -> Result<RawCode> {
    json_parser::process_json_file(filename)
}

/// Reads JSON data from a `&str` and compiles it into `RawCode`
///
/// # Examples
///
/// ```
/// extern crate codespawn;
///
/// let raw_code = codespawn::from_json_str("{\"enum\": { \"name\": \"Foo\",\"var\": {\"name\": \"EnumVal1\",\"type\": \"int\" }}}").unwrap();
/// ```
pub fn from_json_str(json: &str) -> Result<RawCode> {
    json_parser::process_json_str(json)
}

/// Reads Lua data from file and compiles it into `RawCode`
///
/// # Examples
///
/// ```
/// extern crate codespawn;
///
/// let raw_code = codespawn::from_lua("examples/sample.lua").unwrap();
/// ```
pub fn from_lua(filename: &str) -> Result<RawCode> {
    lua_parser::process_lua_file(filename)
}

/// Reads Lua data from a `&str` and compiles it into `RawCode`
///
/// # Examples
///
/// ```
/// extern crate codespawn;
///
/// let raw_code = codespawn::from_lua_str("");
/// ```
pub fn from_lua_str(json: &str) -> Result<RawCode> {
    lua_parser::process_lua_str(json)
}
