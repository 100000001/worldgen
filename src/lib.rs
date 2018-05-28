extern crate noise;

use noise::{NoiseFn, Perlin, Seedable};

pub struct MacroChunk {
    alt: u32,
}

impl MacroChunk {
    pub fn altitude(&self) -> u32 { self.alt }
}

pub struct MacroWorld {
    seed: u32,
    size: u32,
    chunks: Vec<MacroChunk>,
}

impl MacroWorld {
    pub fn new(seed: u32, size: u32) -> MacroWorld {
        let mut chunks = Vec::new();

        let perlin = Perlin::new().set_seed(seed);

        for x in 0..size {
            for y in 0..size {
                let alt = ((perlin.get([x as f64 * 0.1, y as f64 * 0.1]) + 1.) * 128.) as u32;
                chunks.push(MacroChunk {
                    alt,
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
    use super::MacroWorld;

    #[test]
    fn new_world() {
        let _mw = MacroWorld::new(1337, 4);
    }
}
