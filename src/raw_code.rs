pub struct RawCode {
    pub foo: u8,
}

impl RawCode {
    pub fn new() -> RawCode {
        RawCode {
            foo: 0
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
