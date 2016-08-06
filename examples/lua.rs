extern crate codespawn;

fn main()
{
    // generate from XML definition
    let raw_code = codespawn::from_lua("examples/sample.lua").unwrap_or_else(|e| {
        panic!("{}", e);
    });

    println!("\n*** Language specific configs (if defined):");
    for c in raw_code.configs.iter() {
        println!("{}", c.1);
    }

    println!("*** Structure of raw (unformatted) code data:\n{}", raw_code);
    /*println!("*** Structure of C++ code generated from raw code data:\n{}", raw_code.to_cpp().unwrap());
    println!("*** Structure of Rust code generated from raw code data:\n{}", raw_code.to_rust().unwrap());
    println!("*** Generated C++ code:\n\n{}", raw_code.to_cpp().unwrap().to_string());
    println!("*** Generated Rust code:\n\n{}", raw_code.to_rust().unwrap().to_string());

    // save generated code to file
    let _ = raw_code.to_cpp().unwrap().to_file("sample.cpp");
    let _ = raw_code.to_rust().unwrap().to_file("sample.rs");*/
}
