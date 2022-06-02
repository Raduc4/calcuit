use calcuit::main_fuctionality::{add, divide, multiply, parse_the_input_into_f32, subtract};
use std::io;

fn main() {
    loop {
        let mut expresion = String::new();

        io::stdin()
            .read_line(&mut expresion)
            .expect("Failed to read line");

        if expresion.trim() == String::from("quit") {
            std::process::exit(1);
        }
        let parsed_numbers = parse_the_input_into_f32(&expresion);
        println!("{:?}", parsed_numbers);
        println!("\n");
        match expresion {
            exp if exp.contains("+") => {}
            _ => (),
        }
    }
}
