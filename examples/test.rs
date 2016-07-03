extern crate codespawn;

fn main()
{
    let raw_code = codespawn::from_xml("examples/sample.xml");
    for c in raw_code.configs.iter() {
        println!("{}", c.1);
    }
    println!("{}", raw_code);
    let cpp_code  = raw_code.to_cpp();
    let rust_code = raw_code.to_rust();
    println!("{}", rust_code);
    //code_data.to_cpp().to_file("sample.cpp");
    //code_data.to_rust().to_file("sample.rs");
}
