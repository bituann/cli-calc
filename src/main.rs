use std::io;


fn main() {
	println!("A calculator");
	println!("Enter a sequence of calculations witha space after every element e.g 4 + 2 - 3");
	println!("Supported operations: +, -, *, /");
	
	//read input string
	let mut calc_string = String::new();
	
	io::stdin()
		.read_line(&mut calc_string)
		.expect("Cannot read input");
		
	let tokens: Vec<&str> = calc_string.trim()
		.split_whitespace().collect();
		
	println!("{:?}", tokens);
	
	//store result
	let result: f64 = 0.0;
	
	//evaluate expression
	let length = tokens.len();
	let mut i = 0;
	while i < length {
		println!("{}", tokens[i]);
		i += 1;
	}
}

fn add (num1: f64, num2: f64) -> f64 {
	num1 + num2
}

fn subtract (num1: f64, num2: f64) -> f64{
	num1 - num2
}

fn multiply (num1: f64, num2: f64) -> f64 {
	num1 * num2
}

fn divide (num1: f64, num2: f64) -> {
	num1 / num2
}
