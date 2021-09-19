fn main() {
	let mut v: Vec<u64> = vec![];
	v.push(5);
	v.push(2);
	v.push(1);
	println!("{}", v.pop().unwrap());
	println!("{}", v.pop().unwrap());
	println!("{}", v.pop().unwrap());
	println!("{}", v.is_empty());
}
