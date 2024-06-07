use std::fmt;

pub struct BitFrequency {
    count: u32,
    n_bytes: usize,
    bit_freq: Vec<u32>,
}

impl BitFrequency {
    pub fn new(bytes: usize) -> Self {
        Self {
            count: 0_u32,
            n_bytes: bytes,
            bit_freq: vec![0_u32; bytes * 8_usize],
        }
    }
    pub fn clear(&mut self) {
        self.count = 0_u32;
        self.bit_freq.clear();
        self.bit_freq.resize(self.n_bytes * 8, 0_u32);
    }
    pub fn count(&self) -> u32 {
        self.count
    }
    pub fn n_bytes(&self) -> usize {
        self.n_bytes
    }
    pub fn bit_freq(&self) -> &[u32] {
        &self.bit_freq
    }
    pub fn update(&mut self, x: &[u8]) {
        for (byte_i, byte) in x.iter().enumerate() {
            for bit in 0_usize..8_usize {
                let offset = (byte_i * 8_usize) + bit;
                let value = (byte & (0b10000000 >> bit)).count_ones();
                self.bit_freq[offset] += value;
            }
        }
        self.count += 1;
    }
    pub fn count_string(&self, bit_sep: &str, byte_sep: &str) -> String {
        self.bit_freq()
            .chunks(8)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(bit_sep)
            })
            .collect::<Vec<String>>()
            .join(byte_sep)
    }
}

impl Default for BitFrequency {
    fn default() -> Self {
        Self::new(1)
    }
}

impl fmt::Display for BitFrequency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.count_string("\t", "\t"))
    }
}
