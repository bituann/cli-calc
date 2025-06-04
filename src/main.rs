use std::io;


fn main() {
	println!("A calculator");
	println!("Enter an algebraic expression");
	println!("Supported operations: +, -, *, /\n");
	println!("Enter the algebraic expression: ");
	
	//read input string
	let mut calc_string = String::new();
	
	io::stdin()
		.read_line(&mut calc_string)
		.expect("Cannot read input");
		
	//starting operation is + because of order of operation
	println!("\nThe answer is: {}\n", evaluate(&calc_string, '+'));
}

fn evaluate (expression: &String, operation: char) -> f64 {
	//base case
	if check_for_num(expression) {
		return expression.trim().parse::<f64>().unwrap();
	}
	
	//splitexpression into tokens using given operation
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
			let mut result: f64;
			
			//allow for initial result value to be the correct value for the first token
			//raise to power 2 so the base value will be there when the first token is used to divide
			if check_for_num(&tokens[0].to_string()) {
				result = tokens[0].trim().parse::<f64>().unwrap().powi(2);
			} else {
				result = evaluate(&tokens[0].to_string(), '.').powi(2);
			}
			
			for token in tokens {
				result /= evaluate(&token.to_string(), '.');
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
			let mut result:f64;
			
			//allow for initial result value to be the correct value for the first token
			//multiply by 2 so the base value will be there when the first token is subtracted
			if check_for_num(&tokens[0].to_string()) {
				result = tokens[0].trim().parse::<f64>().unwrap() * 2.0;
			} else {
				result = evaluate(&tokens[0].to_string(), '*') * 2.0;
			}
			
			for token in tokens {
				result -= evaluate(&token.to_string(), '*');
			}
			
			return result;
		}
		
		_ => 0.0,
	};
}

//check if a token is a number or an algebraic expression
fn check_for_num (string: &String) -> bool {
	match string.trim().parse::<f64>() {
		Ok(_num) => true,
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
