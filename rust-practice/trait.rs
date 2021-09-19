struct Rect {
	x: i32,
	y: i32,
}

trait Printable {
	fn print(&self);
}

impl Printable for Rect {
	fn print(&self) {
		println!("width:{}, heigth:{}", self.x, self.y);
	}
}

fn main() {
	let r = Rect { x:1, y:3 };
	r.print();
}
