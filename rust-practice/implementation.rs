struct Rect {
	x: i32,
	y: i32,
}

impl Rect {
	fn area(&self) -> i64 {
		(self.x * self.y) as i64
	}
}

fn main() {
	let r = Rect { x:2, y:3 };
	println!("{}", r.area());
}
