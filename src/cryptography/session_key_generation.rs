use sha2::{Digest, Sha256};

pub fn generate_key(buff: &[u8], buff_size: usize, output_buff: &mut [u8], output_buff_size: usize) {
    let half_size = buff_size / 2;

    let mut sh = Sha256::new();
    let mut buff_region = &buff[0..half_size];
    sh.update(&buff_region);

    let mut taken;
    let mut o0 = vec![0; 32];
    let o1 = sh.finalize_reset().to_vec();

    // Take the rest of the buffer and hash it
    buff_region = &buff[half_size..buff_size];
    sh.update(&buff_region);
    let o2 = sh.finalize_reset().to_vec();

    // Fill up the buffer
    sh.update(&o1);
    sh.update(&o0);
    sh.update(&o2);

    o0 = sh.finalize_reset().to_vec();
    taken = 0;

    // generate the key
    for i in 0..output_buff_size {
        if taken == 32 {
            // Fill up the buffer
            sh.update(&o1);
            sh.update(&o0);
            sh.update(&o2);

            o0 = sh.finalize_reset().to_vec();
            taken = 0;
        }
        output_buff[i as usize] = o0[taken];
        taken += 1;
    }
}
