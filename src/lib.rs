pub mod main_fuctionality {
    pub fn add() -> f32 {
        1.4
    }
    pub fn subtract() -> f32 {
        1.4
    }
    pub fn multiply() -> f32 {
        1.4
    }
    pub fn divide() -> f32 {
        1.4
    }
}

#[cfg(test)]
mod test_main_funtionality {
    use super::main_fuctionality::*;
    #[test]
    fn it_works() {
        let add_expect = add();

        assert_eq!(1.4, add_expect);
    }
}
