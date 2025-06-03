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
	let mut result: f64 = 0.0;
	
	//evaluate expression
	let length = tokens.len();
	let mut i = 0;
	
	while i < length {
		//check for operation
		match tokens[i] {
			"+" => {
				let mut num1: f64 = 0.0;
				if result == 0.0 {
					num1 = tokens[i-1].parse()
						.expect("Not a number");
				} else {
					num1 = result;
				}
				
				let num2: f64 = tokens[i+1].parse()
					.expect("Not a number");
					
				result = add(num1, num2);
				println!("{}", result);
			}
			
			"-" => {
				let mut num1: f64 = 0.0;
				if result == 0.0 {
					num1 = tokens[i-1].parse()
						.expect("Not a number");
				} else {
					num1 = result;
				}
				
				let num2 = tokens[i+1].parse()
					.expect("Not a number");
				
				result = subtract(num1, num2);
				println!("{}", result);
			}
			
			&_ => println!(""),
			
		}
		
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

fn divide (num1: f64, num2: f64) -> f64 {
	num1 / num2
}
