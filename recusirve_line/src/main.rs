mod personal_utils;

use personal_utils::get_input;

fn reverse_str(mut input:String) -> String {
	if input.is_empty() {
		input
	}
	else {
		let removed_char = input.remove(0);
		let mut input = reverse_str(input);
		input.push(removed_char);
		input
	
	}
}

fn main() {
    let user_input = get_input();
	println!("{}", reverse_str(user_input));
}