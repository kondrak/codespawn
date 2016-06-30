pub struct RawCodeItem {
    pub name: String,
    pub children: Vec<RawCodeItem>
}

impl RawCodeItem {
    pub fn new(item_name: &str) -> RawCodeItem {
        RawCodeItem {
            name: String::from(item_name),
            children: Vec::<RawCodeItem>::new()
        }
    }
}

pub struct RawCode {
    pub elements: Vec<RawCodeItem>
}

impl RawCode {
    pub fn new() -> RawCode {
        RawCode {
            elements: Vec::<RawCodeItem>::new()
        }
    }

    // convert raw data to cpp code
    pub fn to_cpp(&self) -> u8 {
        0
    }

    // convert raw data to Rust code
    pub fn to_rust(&self) -> u8 {
        0
    }
}
