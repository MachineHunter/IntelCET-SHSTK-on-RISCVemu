macro_rules! log {
	($x:expr) => { println!("{}", $x); }
}

fn main() {
	log!("hello");
}
