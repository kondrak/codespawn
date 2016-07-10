Codespawn
=========

[![Crates.io](https://img.shields.io/crates/v/codespawn.svg)](https://crates.io/crates/codespawn)
[![Build Status](https://travis-ci.org/kondrak/codespawn.svg)](https://travis-ci.org/kondrak/codespawn)
[![Build status](https://ci.appveyor.com/api/projects/status/3pw4g0n398qpud79?svg=true)](https://ci.appveyor.com/project/kondrak/codespawn)
[![Coverage Status](https://coveralls.io/repos/github/kondrak/codespawn/badge.svg?branch=master)](https://coveralls.io/github/kondrak/codespawn?branch=master)

Codespawn is a basic code generator for C++ and Rust. Initially, it was written as a helper tool for [ProDBG](https://github.com/emoon/ProDBG). API definition can be done in either JSON or XML, the crate supports both reading from file or from a string directly in the program. As of 0.1 release, it's possible to generate enums, structs, functions and variables.

Usage
-----
```toml
# Cargo.toml
[dependencies]
codespawn = "0.1"
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
    let cpp_code  = raw_code.to_cpp().to_string();
    let rust_code = raw_code.to_rust().to_string();

    // generate and save directly to file
    raw_code.to_cpp().to_file("sample.cpp");
    raw_code.to_rust().to_file("sample.rs");
}
```

Build instructions
------------------

```
cargo build
cargo run --example xml
```

This will run the [example](https://github.com/kondrak/codespawn/blob/master/examples/xml.rs) which will print all parsed sample definitions and save generated code to files.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
