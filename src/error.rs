extern crate json;
extern crate xml;

use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum CodeSpawnError {
    Io(io::Error),
    Json(json::JsonError),
    Xml(xml::reader::Error)
}

impl fmt::Display for CodeSpawnError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CodeSpawnError::Io(ref err) => err.fmt(f),
            CodeSpawnError::Json(ref err) => err.fmt(f),
            CodeSpawnError::Xml(ref err) => err.fmt(f),
        }
    }
}

impl Error for CodeSpawnError {
    fn description(&self) -> &str {
        match *self {
            CodeSpawnError::Io(ref err) => err.description(),
            CodeSpawnError::Json(ref err) => err.description(),
            CodeSpawnError::Xml(ref err) => err.description(),
        }
    }
}

impl From<io::Error> for CodeSpawnError {
    fn from(err: io::Error) -> CodeSpawnError {
        CodeSpawnError::Io(err)
    }
}

impl From<json::JsonError> for CodeSpawnError {
    fn from(err: json::JsonError) -> CodeSpawnError {
        CodeSpawnError::Json(err)
    }
}

impl From<xml::reader::Error> for CodeSpawnError {
    fn from(err: xml::reader::Error) -> CodeSpawnError {
        CodeSpawnError::Xml(err)
    }
}
