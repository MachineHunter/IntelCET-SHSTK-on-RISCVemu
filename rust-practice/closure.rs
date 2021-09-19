fn main() {
	let f = |x:i32, y:i32| { x+y };
	println!("{}", f(2,3));

	let msg = String::from("hoge");
	let g = move || {
		println!("{}", msg);
	};
	g();
}
