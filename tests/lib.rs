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
fn check_simple_json() {
    let raw_code = codespawn::from_json_str("{\"config\": [\"tests/dummy_cfg.json\", \"examples/config.json\"],\"var\":[{\"void*\":\"void_ptr\"},{\"int\":\"some_number\"}]}").unwrap();
    let _ = raw_code.to_cpp();
    let _ = raw_code.to_rust();
}

#[test]
#[should_panic]
fn check_from_xml_fail() {
    let _ = codespawn::from_xml("foobar").unwrap(); }

#[test]
#[should_panic]
fn check_from_json_fail() {
    let _ = codespawn::from_json("foobar").unwrap(); }

#[test]
#[should_panic]
fn check_from_xml_fail_cfg() {
    let _ = codespawn::from_xml_str("<config name=\"foobar\"/><var name=\"x\" type=\"int\"/>").unwrap(); }

#[test]
#[should_panic]
fn check_from_json_fail_cfg() {
    let _ = codespawn::from_json_str("{\"config\": \"foobar\", \"var\": {\"name\": \"x\", \"type\":\"int\" }}").unwrap(); }

#[test]
#[should_panic]
fn check_from_xml_fail_malformed() {
    let _ = codespawn::from_xml_str("<config name=\"foobar\"/>var name=\"x\" type=\"int\"/>").unwrap(); }

#[test]
#[should_panic]
fn check_from_json_fail_malformed() {
    let _ = codespawn::from_json_str("{\"config\": \"examples/config.json\" \"var\": {\"name\": \"x\", \"type\":\"int\" }}").unwrap(); }

#[test]
#[should_panic]
fn check_from_xml_fail_enum() {
    let rc = codespawn::from_xml_str("<enum name=\"Foo\"><func name=\"x\" type=\"int\"/></enum>").unwrap();
    rc.to_rust().to_string(); }

#[test]
#[should_panic]
fn check_from_json_fail_enum() {
    let rc = codespawn::from_json_str("{\"enum\":{ \"name\":\"Foo\",\"func\": { \"name\":\"x\",\"type\":\"int\"}}}").unwrap();
    rc.to_cpp().to_string(); }

#[test]
#[should_panic]
fn check_from_xml_fail_func() {
    let rc = codespawn::from_xml_str("<func name=\"x\" type=\"int\"><enum name=\"Foo\"></enum></func>").unwrap();
    rc.to_rust().to_string(); }

#[test]
#[should_panic]
fn check_from_json_fail_func() {
    let rc = codespawn::from_json_str("{\"func\": {\"name\":\"x\",\"type\":\"int\",\"enum\": {\"name\":\"Foo\"}}}").unwrap();
    rc.to_cpp().to_string(); }

#[test]
#[should_panic]
fn check_from_xml_fail_bitflags() {
    let rc = codespawn::from_xml_str("<struct name=\"Foo\"><bitflags name=\"x\" type=\"int\"><enum></enum></bitflags></struct>").unwrap();
    rc.to_rust().to_string(); }

#[test]
#[should_panic]
fn check_from_json_fail_bitflags() {
    let rc = codespawn::from_json_str("{\"struct\":{ \"name\":\"Foo\",\"bitflags\": { \"name\":\"x\",\"type\":\"int\", \"enum\": { \"name\": \"foonum\" }}}}").unwrap();
    rc.to_rust().to_string(); }

#[test]
#[should_panic]
fn check_from_xml_fail_malformed_cfg() {
    let _ = codespawn::from_xml_str("<config name=\"tests/malformed_cfg.xml\"/><var name=\"x\" type=\"int\"/>").unwrap(); }

#[test]
#[should_panic]
fn check_write_file() {
    let raw_code = codespawn::from_xml("examples/sample.xml").unwrap();
    raw_code.to_cpp().to_file("").unwrap(); }
