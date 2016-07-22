extern crate codespawn;

fn main()
{
    // Example of parsing a simplified JSON definition.
    // 'name' and 'type' can be omitted and will be automatically deduced by the parser.
    let raw_code = codespawn::from_json("examples/sample2.json").unwrap();

    println!("\n*** Language specific configs (if defined):");
    for c in raw_code.configs.iter() {
        println!("{}", c.1);
    }

    println!("*** Structure of raw (unformatted) code data:\n{}", raw_code);
    println!("*** Structure of C++ code generated from raw code data:\n{}", raw_code.to_cpp());
    println!("*** Structure of Rust code generated from raw code data:\n{}", raw_code.to_rust());
    println!("*** Generated C++ code:\n\n{}", raw_code.to_cpp().to_string());
    println!("*** Generated Rust code:\n\n{}", raw_code.to_rust().to_string());

    // save generated code to file
    let _ = raw_code.to_cpp().to_file("sample2.cpp");
    let _ = raw_code.to_rust().to_file("sample2.rs");
}
