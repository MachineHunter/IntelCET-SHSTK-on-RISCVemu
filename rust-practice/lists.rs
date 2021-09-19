fn main() {
	let arr = [1,2,3];
	println!("{:?} {}", &arr[1..2], arr[0]);

	let tup = (1, "2", 3);
	println!("{} {}", tup.1, tup.2);

	let mut v = vec![1,2];
	v.push(3);
	println!("{}", v.pop().unwrap());
	for i in &v {
		println!("{}", i);
	}

	use std::collections::HashMap;
	let mut map = HashMap::new();
	map.insert("x", 10);
	map.insert("y", 20);
	println!("{} {}", map["x"], map["y"]);
	for (k, v) in &map {
		println!("{} {}", k, v);
	}
}
