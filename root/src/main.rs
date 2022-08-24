fn calculate_fith_root(result:f64) -> f64 {
	result.powf(1 as f64 / 5 as f64)
}

fn main() {
	let mut result = 0;

    for i in 0..100 {
		if i % 2 == 0 {
			result += i * i;
		}
	}
	println!("The fith root of the sum of all odd number to 100 is {}", calculate_fith_root(result as f64));
}