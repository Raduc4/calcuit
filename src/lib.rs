pub type Numbers = (f32, f32);
pub struct Final<'a> {
    numbers: Vec<f32>,
    sign: &'a str,
}
impl Final {
    pub fn new<'a>(&self, numbers: Vec<f32>, sign: &'a str) -> Self {
        Self {
            numbers: numbers,
            sign: &sign,
        }
    }
}
pub mod main_fuctionality {
    use std::ops::{Add, Div, Mul, Sub};

    pub fn parse_the_input_into_f32<'a>(expr: &'a str) -> Vec<f32> {
        let mut stack: Vec<String> = vec![];

        for c in expr.chars() {
            stack.push(c.to_string());
        }
        stack.pop();
        let mut returning_vec_with_numbers: Vec<f32> = vec![];
        stack.iter().for_each(|item| {});
        returning_vec_with_numbers
    }
    pub fn add<T>(n1: T, n2: T) -> T
    where
        T: Add<Output = T>,
    {
        n1 + n2
    }
    pub fn subtract<T>(n1: T, n2: T) -> T
    where
        T: Sub<Output = T>,
    {
        n1 - n2
    }
    pub fn multiply<T>(n1: T, n2: T) -> T
    where
        T: Mul<Output = T>,
    {
        n1 * n2
    }
    pub fn divide<T>(n1: T, n2: T) -> T
    where
        T: Div<Output = T>,
    {
        n1 / n2
    }
}

#[cfg(test)]
mod test_main_funtionality {
    use super::main_fuctionality::*;
    #[test]
    fn add_test() {
        let add_expect = add(1.0, 0.4);

        assert_eq!(1.4, add_expect);
    }
    #[test]
    fn subtract_test() {
        let subtract_expect = subtract(1.0, 0.4);
        assert_eq!(0.6, subtract_expect);
    }
    #[test]
    fn multiply_test() {
        let multiply_test = multiply(1.0, 0.4);
        assert_eq!(0.4, multiply_test);
    }
    #[test]
    fn divide_test() {
        let divide_expect = divide(1.0, 0.4);
        assert_eq!(2.5, divide_expect);
    }
    #[test]
    fn divide_tes2t() {
        let divide_expect = divide(142.20, 2.0);
        assert_eq!(71.1, divide_expect);
    }
    #[test]
    fn parse_strings_input() {
        let parsing = parse_the_input_into_f32("1 + 1");
        assert_eq!(vec![1.0, 1.0], parsing);
    }
}
