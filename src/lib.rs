pub struct MacroChunk {
	alt: u32,
}

pub struct MacroWorld {
	seed: u64,
	size: u32,
	chunks: Vec<MacroChunk>,
}

impl MacroWorld {
	pub fn new(seed: u64, size: u32) -> MacroWorld {
		let mut chunks = Vec::new();

		for i in 0..size * size {
			chunks.push(MacroChunk {
				alt: 0,
			});
		}

		MacroWorld {
			seed,
			size,
			chunks,
		}
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
