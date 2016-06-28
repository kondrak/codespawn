extern crate codespawn;

fn main()
{
    let code_data = codespawn::from_xml("examples/sample.xml");
    let cpp_code  = code_data.to_cpp();
    let rust_code = code_data.to_rust();
    //code_data.to_cpp().to_file("sample.cpp");
    //code_data.to_rust().to_file("sample.rs");
}
