pub struct ShadowStack {
	data: Vec<u64>
}

impl ShadowStack {
	pub fn new() -> Self {
		ShadowStack {
			data: vec![]
		}
	}

	pub fn push(&mut self, value: u64) -> bool {
		self.data.push(value);
		return true;
	}

	pub fn pop(&mut self) -> u64 {
		return self.data.pop().unwrap();
	}
}
