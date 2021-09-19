// https://www.tohoho-web.com/ex/rust.html

fn add(x:i32, y:i32) -> i32 {
	return x+y;
}

fn main() {
	let a = 0;              // aは変更不可能
	let mut b = 0;          // bは変更可能(ミュータブル)
	let c: i16 = 1;         // 型をint(16Byte)に指定
	let d: i32 = c as i32;  // 型変換 i16=>i32
	let e = [1,2,3];
	let f = if a==0 {"Ok"} else {"False"};
	for i in 0..5 { b+=i };
	
	println!("{} {} {} {} {} {} {}", a, b, c, d, e[1], add(2,3), f);
}
