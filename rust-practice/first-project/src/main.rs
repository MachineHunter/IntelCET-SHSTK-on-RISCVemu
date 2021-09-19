mod hoge;
use hoge::*;
mod foo;

fn main() {
	print();
	foo::bar::pprint();
}
