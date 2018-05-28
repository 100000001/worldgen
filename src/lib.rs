extern crate noise;

use noise::{NoiseFn, Perlin};

pub struct MacroChunk {
    alt: u32,
}

impl MacroChunk {
    pub fn altitude(&self) -> u32 { self.alt }
}

pub struct MacroWorld {
    seed: u64,
    size: u32,
    chunks: Vec<MacroChunk>,
}

impl MacroWorld {
    pub fn new(seed: u64, size: u32) -> MacroWorld {
        let mut chunks = Vec::new();

        let perlin = Perlin::new();

        for x in 0..size {
            for y in 0..size {
                chunks.push(MacroChunk {
                    alt: ((perlin.get([x as f64, y as f64]) + 1.) * 256.) as u32,
                });
            }
        }

        MacroWorld {
            seed,
            size,
            chunks,
        }
    }

    pub fn size(&self) -> u32 { self.size }

    pub fn get<'a>(&'a self, x: u32, y: u32) -> Option<&'a MacroChunk> {
        self.chunks.get(self.size as usize * x as usize + y as usize)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
