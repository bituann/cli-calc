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
		
	println!("{}\n", evaluate(&calc_string, '*'));
		
	let mult_tokens: Vec<&str> = calc_string.trim()
		.split('*').collect();
		
	println!("{:?}", mult_tokens);
	
	let mut result = 0.0;
	
	for mult_token in mult_tokens {
		//handle division
		let div_tokens: Vec<&str> = mult_token.trim()
			.split('/').collect();
	}
	
	
	/*
	//store result
	let mut result: f64 = 0.0;
	
	//evaluate expression
	let length = tokens.len();
	let mut i = 0;
	
	//order of operations taken into account
	
	//basic calculation alone, to be removed
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
			
			"*" => {
				let mut num1: f64;
				if result == 0.0 {
					num1 = tokens[i-1].parse()
						.expect("Not a number");
				} else {
					num1 = result;
				}
				
				let num2: f64 = tokens[i+1].parse()
					.expect("Not a number");
				
				result = multiply(num1, num2);
				println!("{}", result);
			}
			
			"/" => {
				let mut num1: f64;
				if result == 0.0 {
					num1 = tokens[i-1].parse()
						.expect("Not a number");
				} else {
					num1 = result;
				}
				
				let num2: f64 = tokens[i+1].parse()
					.expect("Not a number");
				
				result = divide(num1, num2);
				println!("{}", result);
			}
			
			&_ => println!(""),
		}
		
		i += 1;
	}
	*/
}

fn evaluate (expression: &String, operation: char) -> f64 {
	if check_for_num(expression) {
		return expression.trim().parse::<f64>().unwrap();
	}
	
	let tokens: Vec<&str> = expression.trim()
		.split(operation).collect();
		
	match operation {
		'*' => {
			let mut result: f64 = 1.0;
			
			for token in tokens {
				result *= evaluate(&token.to_string(), '/');
			}
			
			return result;
		}
		
		'/' => {
			let mut result: f64 = tokens[0].trim().parse::<f64>().unwrap().powi(2);
			
			for token in tokens {
				result /= evaluate(&token.to_string(), '+');
			}
			
			return result;
		}
		
		'+' => {
			let mut result: f64 = 0.0;
			
			for token in tokens {
				result += evaluate(&token.to_string(), '-');
			}
			
			return result;
		}
		
		'-' => {
			let mut result: f64 = tokens[0].trim().parse::<f64>().unwrap() * 2.0;
			
			for token in tokens {
				result -= evaluate(&token.to_string(), '*');
			}
			
			return result;
		}
		
		_ => 0.0,
	};
	
	return 0.0
}

fn check_for_num (string: &String) -> bool {
	match string.trim().parse::<f64>() {
		Ok(num) => true,
		Err(_) => false,
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
