use sha2::{Sha256, Digest};

pub struct SessionKeyGenerator<'a> {
    pub o0: &'a[u8] ,
    pub o1: &'a[u8],
    pub o2: &'a[u8],
    pub taken: u32,
    pub sh: Sha256,
}

impl<'a> SessionKeyGenerator<'a> {
    pub fn new(buff: &[u8], size: usize) -> Self {
        let mut key_generator = Self {
            o0: &[0; 32],
            o1: &[0; 32],
            o2: &[0; 32],
            taken: 0,
            sh: Sha256::new(),
        };
        // let half_size = size / 2;
        // key_generator.o1 = transform_final_block(&buff, 0, half_size);
        // key_generator.o2 = transform_final_block(buff, half_size, size - half_size);

        key_generator
    }

    // pub fn generate_key(buff: &[u8], size: i32, sz: u32) -> Result<[u8], Err> {
    //
    // }
}

// Computes the hash value for the specified region of the input byte array and copies it
// to the specified region of the returned byte array.
// TODO: Update to be more efficient by passing a mutable reference to the output byte array
// TODO: Dig into C# version of this function to ensure output is correct
pub fn transform_block(input_buffer: &[u8], input_offset: usize, input_count: usize, output_offset: usize) -> Vec<u8> {
    let mut hasher = Sha256::new();
    let buff_region = &input_buffer[input_offset..input_count];
    let mut out_buffer = input_buffer.to_vec();

    hasher.update(buff_region);

    let mut i = 0;
    let final_hash = hasher.finalize();
    let hash_slice = &final_hash[..];
    println!("{:?}", hash_slice);
    while i < hash_slice.len() {
        if output_offset + i < out_buffer.len() {
            r[output_offset + i] = hash_slice[i];
        }
        i = i + 1;
    }

    out_buffer.try_into().expect("slice with incorrect length")
}

// Computes the hash value for the specified region of the given byte array.
// TODO: Check if we can do this without allocating a temp buffer or hasher.
// fn transform_final_block(buff: &[u8], input_offset: usize, input_count: usize) -> &[u8] {
//     let mut hasher = Sha256::new();
//     let buff_region = &buff[input_offset..input_count];
//     hasher.update(buff_region);
//
//     hasher.finalize().to_bytes()
// }