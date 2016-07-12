//! [Codespawn](https://github.com/kondrak/codespawn) is a basic C++ and Rust code generator.
//! Desired API can be defined using either JSON or XML and the crate supports both reading
//! from a file or a string.
//!
//! As of 0.1 release, it's possible to generate enums, structs, functions and variables with all
//! applicable attributes and properties.
//!
//! See [example XML](https://github.com/kondrak/codespawn/blob/master/examples/sample.xml) for instructions on how
//! to construct the API definition.
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
//!    let cpp_code  = raw_code.to_cpp().to_string();
//!    let rust_code = raw_code.to_rust().to_string();
//!
//!    // generate C++ and save directly to file
//!    raw_code.to_cpp().to_file("examples/sample.cpp");
//!    // generate Rust and save directly to file
//!    //raw_code.to_rust().to_file("examples/sample.rs");
//!}
//!```
mod xml_parser;
mod json_parser;
mod string_gen;

pub mod raw_code;
pub mod fmt_code;

use std::io;
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
pub fn from_xml(filename: &str) -> io::Result<RawCode> {
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
pub fn from_xml_str(xml: &str) -> io::Result<RawCode> {
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
pub fn from_json(filename: &str) -> io::Result<RawCode> {
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
pub fn from_json_str(json: &str) -> io::Result<RawCode> {
    json_parser::process_json_str(json)
}
