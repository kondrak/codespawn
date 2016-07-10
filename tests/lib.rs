extern crate codespawn;

#[test]
fn check_from_xml() {
    let raw_code = codespawn::from_xml("examples/sample.xml").unwrap();

    for c in raw_code.configs.iter() {
        println!("{}", c.1);
    }

    println!("{}", raw_code);
    println!("{}", raw_code.to_cpp());
    println!("{}", raw_code.to_rust());

    // save to .rst file instead of .rs so that subsequent test don't get confused
    let _ = raw_code.to_rust().to_file("tests/sample.rst");
    let _ = raw_code.to_cpp().to_file("tests/sample.cpp");
}

#[test]
fn check_from_json() {
    let raw_code = codespawn::from_json("examples/sample.json").unwrap();

    for c in raw_code.configs.iter() {
        println!("{}", c.1);
    }

    println!("{}", raw_code);
    println!("{}", raw_code.to_cpp());
    println!("{}", raw_code.to_rust());

    // save to .rst file instead of .rs so that subsequent test don't get confused
    let _ = raw_code.to_rust().to_file("tests/sample.rst");
    let _ = raw_code.to_cpp().to_file("tests/sample.cpp");
}

#[test]
#[should_panic]
fn check_from_xml_fail() {
    let _ = codespawn::from_xml("foobar").unwrap();
}

#[test]
#[should_panic]
fn check_from_json_fail() {
    let _ = codespawn::from_json("foobar").unwrap();
}

#[test]
#[should_panic]
fn check_from_xml_fail_cfg() {
    let _ = codespawn::from_xml_str("<config file=\"foobar\"/><var name=\"x\" type=\"int\"/>").unwrap();
}

#[test]
#[should_panic]
fn check_from_json_fail_cfg() {
    let _ = codespawn::from_json_str("{\"config\": { \"file\": \"foobar\" }, \"var\": {\"name\": \"x\", \"type\":\"int\" }}").unwrap();
}

#[test]
#[should_panic]
fn check_from_xml_fail_malformed() {
    let _ = codespawn::from_xml_str("<config file=\"foobar\"/>var name=\"x\" type=\"int\"/>").unwrap();
}

#[test]
#[should_panic]
fn check_from_xml_fail_malformed_cfg() {
    let _ = codespawn::from_xml_str("<config file=\"tests/malformed_cfg.xml\"/>var name=\"x\" type=\"int\"/>").unwrap();
}

#[test]
#[should_panic]
fn check_from_json_fail_malformed() {
    let _ = codespawn::from_json_str("{\"config\": { \"file\": \"examples/rust.json\" } \"var\": {\"name\": \"x\", \"type\":\"int\" }}").unwrap();
}
