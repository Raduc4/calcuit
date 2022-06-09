mod parse;
use parse::ast;
use parse::parser::{ParseError, Parser};
use std::io;
fn main() {
	println!("Hello! Welcome to Arithmetic expression evaluator.");
	println!("You can calculate value for expression such as 2*3+(4-5)+2^3/4. ");
	println!("Allowed numbers: positive, negative and decimals.");
	println!(
		"Supported operations: Add, Subtract, MultiplyPutting it all together 63 Divide, PowerOf(^). "
	);
	println!("Enter your arithmetic expression below:");
	loop {
		let mut input = String::new();
		match io::stdin().read_line(&mut input) {
			Ok(_) => {
				match evaluate(input) {
					Ok(val) => println!("The computed number is {}\n", val),
					Err(_) => {
						println!("Error in evaluating expression. Please enter valid expression\n");
					}
				};
			}
			Err(error) => println!("error: {}", error),
		}
	}
}

fn evaluate(expr: String) -> Result<f64, ParseError> {
	let expr = expr.split_whitespace().collect::<String>();
	// remove whitespace chars
	let mut math_parser = Parser::new(&expr)?;
	let ast = math_parser.parse()?;
	println!("The generated AST is {:?}", ast);
	Ok(ast::eval(ast)?)
}
