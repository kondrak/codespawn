#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        ::foo();
    }
}

pub fn foo()
{
    println!("Hello world!");
}