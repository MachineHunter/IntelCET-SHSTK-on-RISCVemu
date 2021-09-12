pub struct ShadowStack {
	data: Vec<u64>
}

impl ShadowStack {
	pub fn new() -> Self {
		ShadowStack {
			data: vec![]
		}
	}

	pub fn push(&mut self, value: u64) -> Result<bool, String> {
		data.push(value);
		return Ok(true);
	}

	pub fn pop(&mut self) -> Result<u64, String> {
		if data.is_empty() {
			return Err(String::from("ShadowStack underflow"));
		}
		return Ok(data.pop().unwrap());
	}
}
