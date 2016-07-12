extern crate codespawn;

fn main()
{
    // generate from XML definition
    let raw_code = codespawn::from_xml("examples/sample.xml").unwrap();
    // generate from JSON definition
    //let raw_code = codespawn::from_json("examples/sample.json").unwrap();

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
    let _ = raw_code.to_cpp().to_file("sample.cpp");
    let _ = raw_code.to_rust().to_file("sample.rs");
}
