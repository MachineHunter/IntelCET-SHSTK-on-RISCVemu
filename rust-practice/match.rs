fn main() {
	let x = 2;
	match x {
		0..=2 => println!("0 to 2"),
		3 => println!("three"),
		_ => println!("default"),
	}
}

// 0 to 2
