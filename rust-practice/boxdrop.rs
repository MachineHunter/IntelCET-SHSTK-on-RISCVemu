struct Hoge {
	x: u64,
	y: i32,
}

fn main() {
	let h: Box<Hoge> = Box::new( Hoge { x:1, y:2 } );
	println!("{} {}", h.x, h.y);
}

// デストラクタ
impl Drop for Hoge {
	fn drop(&mut self) {
		println!("Bye!");
	}
}

// 結果
// 1 2
// Bye!

