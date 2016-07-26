//! Error type for codespawn crate.
extern crate json;
extern crate xml;

use std::error::Error;
use std::fmt;
use std::num;
use std::io;
use std::result;

macro_rules! some {
    ($x:expr, $msg:expr) => (try!($x.ok_or(CodeSpawnError::Other($msg.to_owned()))))
}

macro_rules! some_str {
    ($x:expr) => (some!($x, "Unable to fetch string."))
}

macro_rules! some_get {
    ($x:expr) => (some!($x, "Unable to fetch object."))
}

/// Result type used throughout the crate.
pub type Result<T> = result::Result<T, CodeSpawnError>;

/// Error type for codespawn crate.
#[derive(Debug)]
pub enum CodeSpawnError {
    /// I/O error
    Io(io::Error),
    /// JSON parser error
    Json(json::JsonError),
    /// XML parser error
    Xml(xml::reader::Error),
    /// Any other kind of error
    Other(String)
}

impl fmt::Display for CodeSpawnError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CodeSpawnError::Io(ref err)    => err.fmt(f),
            CodeSpawnError::Json(ref err)  => err.fmt(f),
            CodeSpawnError::Xml(ref err)   => err.fmt(f),
            CodeSpawnError::Other(ref err) => err.fmt(f)
        }
    }
}

impl Error for CodeSpawnError {
    fn description(&self) -> &str {
        match *self {
            CodeSpawnError::Io(ref err)    => err.description(),
            CodeSpawnError::Json(ref err)  => err.description(),
            CodeSpawnError::Xml(ref err)   => err.description(),
            CodeSpawnError::Other(ref err) => err
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

impl From<String> for CodeSpawnError {
    fn from(err: String) -> CodeSpawnError {
        CodeSpawnError::Other(err)
    }
}

impl From<num::ParseIntError> for CodeSpawnError {
    fn from(err: num::ParseIntError) -> CodeSpawnError {
        CodeSpawnError::Other(err.description().to_owned())
    }
}
