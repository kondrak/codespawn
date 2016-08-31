Codespawn
=========

[![Crates.io](https://img.shields.io/crates/v/codespawn.svg)](https://crates.io/crates/codespawn)
[![Documentation](https://docs.rs/codespawn/badge.svg)](https://docs.rs/codespawn)
[![Build Status](https://travis-ci.org/kondrak/codespawn.svg)](https://travis-ci.org/kondrak/codespawn)
[![Build status](https://ci.appveyor.com/api/projects/status/3pw4g0n398qpud79?svg=true)](https://ci.appveyor.com/project/kondrak/codespawn)
[![Coverage Status](https://coveralls.io/repos/github/kondrak/codespawn/badge.svg?branch=master)](https://coveralls.io/github/kondrak/codespawn?branch=master)
![](https://img.shields.io/crates/l/json.svg)

Codespawn is a basic C++ and Rust code generator. Desired API can be defined using either JSON or XML and the crate supports both reading from a file or a string. Currently it's possible to generate enums, structs, functions, function pointers, variables and bitflags with all applicable attributes and properties.

This crate was created as a helper tool for [ProDBG](https://github.com/emoon/ProDBG). See [example XML](https://github.com/kondrak/codespawn/blob/master/examples/sample.xml) for instructions on how to construct the API definition.

[Documentation](https://docs.rs/codespawn)

Usage
-----
```toml
# Cargo.toml
[dependencies]
codespawn = "0.3"
```

Example
-------
```rust
extern crate codespawn;

fn main()
{
    // generate from XML definition
    let raw_code = codespawn::from_xml("examples/sample.xml").unwrap();
    // generate from JSON definition
    //let raw_code = codespawn::from_json("examples/sample.json").unwrap();

    // generate code, store as String
    let cpp_code  = raw_code.to_cpp().unwrap().to_string();
    let rust_code = raw_code.to_rust().unwrap().to_string();

    // generate and save directly to file
    raw_code.to_cpp().unwrap().to_file("sample.cpp");
    raw_code.to_rust().unwrap().to_file("sample.rs");
}
```

Build instructions
------------------

```
cargo build
cargo run --example xml
cargo run --example json
```

This will run the [example](https://github.com/kondrak/codespawn/blob/master/examples/xml.rs) which will generate code and save it to files using sample XML definition.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
