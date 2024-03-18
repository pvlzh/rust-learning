use std::collections::HashMap;

fn main() {
	let input_value = "MMXXIV";
	let result = roman_to_int(input_value);
	println!("{result}"); // 2024
}

/// converting Roman numbers to integer
pub fn roman_to_int(input: &str) -> i32 {
	let values = HashMap::from([
		('I', 1), ('V', 5),
		('X', 10), ('L', 50),
		('C', 100), ('D', 500),
		('M', 1000),
	]);

	let mut result = 0;
	let input_length = input.len();

	for i in 0..input_length {

		let ch = input.chars().nth(i).unwrap();
		let current = values[&ch];

		let next = if i + 1 < input_length { 
			let ch = input.chars().nth(i + 1).unwrap();
			values[&ch]
		} 
		else { 0 };

		result = if current < next { result - current } else { result + current };
	}
	return result;
}
