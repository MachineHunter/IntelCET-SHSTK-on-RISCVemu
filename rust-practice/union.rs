union Hoge {
	x: u64,
	y: i32,
}

fn main() {
	let h = Hoge { x: 100 };
	unsafe {
		println!("{}", h.x);
		println!("{}", h.y);
	}
}
