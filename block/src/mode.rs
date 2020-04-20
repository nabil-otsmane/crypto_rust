use crate::AES;

pub enum Mode {
    Ecb,
    Cbc([u8; 16])
}

impl Mode {
    pub fn encrypt(self, aes: AES, plain: &[u8]) -> Vec<u8> {
        if plain.len() % 16 != 0 {
            panic!("padding not yet implemented!");
        }
        let mut crypt = vec![];
        match self {
            Self::Ecb => {
                let mut block = [0; 16];
                for i in plain.chunks(16) {
                    for j in 0..16 {
                        block[j] = i[j];
                    }
                    for j in aes.encrypt_block(block).iter() {
                        crypt.push(*j);
                    }
                }
            },
            Self::Cbc(iv) => {
                let mut block = [0; 16];
                let mut y = iv;
                for i in plain.chunks(16) {
                    for j in 0..16 {
                        block[j] = i[j] ^ y[j as usize];
                    }
                    y = aes.encrypt_block(block);
                    for j in y.iter() {
                        crypt.push(*j);
                    }
                }
            }
        }
        crypt
    }

    pub fn decrypt(self, aes: AES, crypt: &[u8]) -> Vec<u8> {
        if crypt.len() % 16 != 0 {
            panic!("padding not yet implemented!");
        }
        let mut plain = vec![];
        match self {
            Self::Ecb => {
                let mut block = [0; 16];
                for i in crypt.chunks(16) {
                    for j in 0..16 {
                        block[j] = i[j];
                    }
                    for j in aes.decrypt_block(block).iter() {
                        plain.push(*j);
                    }
                }
            },
            Self::Cbc(iv) => {
                let mut block;
                let mut y = iv;
                for i in crypt.chunks(16) {
                    let tmp = y;
                    for j in 0..16 {
                        y[j] = i[j];
                    }
                    block = aes.decrypt_block(y);
                    for j in 0..16 {
                        block[j] = block[j] ^ tmp[j];
                    }
                    for j in block.iter() {
                        plain.push(*j);
                    }
                }
            }
        }
        plain
    }
}



#[test]
fn correct_encrypt() {
    let key = [0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
    let input = [0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34];

    let res = [0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb, 0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a, 0x0b, 0x32];

    let aes = AES::new(&key).expect("okaaaaaaay");
    assert_eq!(res, aes.encrypt_block(input));
}

#[test]
fn correct_decrypt() {
    let key = [0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
    let input = [0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34];

    let res = [0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb, 0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a, 0x0b, 0x32];

    let aes = AES::new(&key).expect("okaaaaaaay");
    assert_eq!(input, aes.decrypt_block(res));
}

#[test]
fn correct_encrypt_cbc() {
    let key = [0x06, 0xa9, 0x21, 0x40, 0x36, 0xb8, 0xa1, 0x5b, 0x51, 0x2e, 0x03, 0xd5, 0x34, 0x12, 0x00, 0x06];
    let iv = [0x3d, 0xaf, 0xba, 0x42, 0x9d, 0x9e, 0xb4, 0x30, 0xb4, 0x22, 0xda, 0x80, 0x2c, 0x9f, 0xac, 0x41];

    let plain = b"Single block msg";
    let cipher = vec![0xe3, 0x53, 0x77, 0x9c, 0x10, 0x79, 0xae, 0xb8, 0x27, 0x08, 0x94, 0x2d, 0xbe, 0x77, 0x18, 0x1a];

    let aes = AES::new(&key).expect("Hello");
    assert_eq!(cipher, Mode::Cbc(iv).encrypt(aes, plain));

    
}