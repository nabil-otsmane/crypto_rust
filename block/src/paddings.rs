
pub fn pkcs7(input: &[u8], block_size: u8) -> Vec<u8> {
    let mut res = input.to_vec();
    let pad = block_size - (res.len() as u8 % block_size);
    for _ in 0..pad {
        res.push(pad);
    }
    res
}