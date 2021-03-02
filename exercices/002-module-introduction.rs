mod foo {
	pub fn hello(arg: &str) {
		println!("Hello, {}!", arg);
	}
}

fn main() {
	foo::hello(&"world".to_owned());
}