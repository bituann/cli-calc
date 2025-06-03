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
}
