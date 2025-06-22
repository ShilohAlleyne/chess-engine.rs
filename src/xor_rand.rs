// This needs to persist throughout the lifetime of the program
#[derive(Debug)]
pub struct XorRand {
    init: u32,
    state: u32,
}

impl XorRand {
    fn new() -> Self {
        Self {
            init: 1804289383,
            state: 0,
        }
    }

    // Pseudo rand num gen
    pub fn rand32(&mut self) -> u32 {
        if self.state == 0 {
            self.state = self.init;
        }

        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;

        self.state
    }

    pub fn rand64(&mut self) -> u64 {
        let n: u64 = self.rand32() as u64 & 0xFFFF;
        let n1: u64 = self.rand32() as u64 & 0xFFFF;
        let n2: u64 = self.rand32() as u64 & 0xFFFF;
        let n3: u64 = self.rand32() as u64 & 0xFFFF;

        n | (n1 << 16) | (n2 << 32) | (n3 << 48)
    }

    pub fn gen_magic_number(&mut self) -> u64 {
        self.rand64() & self.rand64() & self.rand64()
    }
}
