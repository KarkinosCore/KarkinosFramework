use sha2::{Sha256, Digest};

pub struct SessionKeyGenerator {
    pub o0: Vec<u8> ,
    pub o1: Vec<u8>,
    pub o2: Vec<u8>,
    pub taken: usize,
}

impl SessionKeyGenerator {
    pub fn new(buff: &[u8], size: usize) -> Self {
        let half_size = size / 2;

        // Take the first half fo the buffer and hash it.
        let mut sh = Sha256::new();
        let mut buff_region = &buff[0..half_size];
        sh.update(&buff_region);

        // Do the actual hashing and clear the hash state, storing the Vec<u8> of the hash.
        let o1 = sh.finalize_reset().to_vec();

        // Take the rest of the buffer and hash it
        buff_region = &buff[half_size..size];
        sh.update(&buff_region);
        let o2 = sh.finalize().to_vec();

         let mut key_gen = Self {
            o0: vec![0; 32],
            o1,
            o2,
            taken: 0,
        };

        key_gen.fill_up();

        key_gen

    }

    // TODO: Possible optimization, take a Sha256 ref vs allocating new one
    fn fill_up(&mut self) {
        let sh = Sha256::new();
        let hash = sh.chain_update(&self.o1).chain_update(&self.o0).chain_update(&self.o2).finalize();
        self.o0 = hash.to_vec();
        self.taken = 0;
    }

    // Generate a key from a given buffer and size.
    // TODO: Might be more Rusty and clear to return a new &[u8] or Vec<u8> instead of modifying the passed buffer
    pub fn generate_key(&mut self, buff: &mut [u8], sz: u8) {
        // TODO: Figure out a more Rusty way of doing this looping
        for i in 0..sz {
            if self.taken == 32 {
                self.fill_up();
            }
            buff[i as usize] = self.o0[self.taken];
        }
    }
}