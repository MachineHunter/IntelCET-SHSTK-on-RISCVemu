fn main() {
	let mut hoge: &str = "hoge";
	hoge = "fuga";
	println!("{}", hoge);

	// ライブラリを使う方法
	let mut aaa = String::from("hoge");
	aaa = "bbb".to_string();
	aaa.push_str(" ccc");
	println!("{}", aaa);
}
