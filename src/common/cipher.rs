pub struct XorCipher {
    key: [u8; 16],
}

impl XorCipher {
    pub fn new(keystr: &[u8]) -> Self {
        let mut key = [0u8; 16];
        key.copy_from_slice(md5::compute(keystr).as_slice());

        XorCipher { key }
    }

    #[inline]
    fn in_place(&mut self, data: &mut [u8]) {
        for i in 0..data.len() {
            data[i] ^= self.key[i % 16];
        }
    }

    #[inline]
    pub fn encrypt_slice(&mut self, plaintext_and_ciphertext: &mut [u8]) {
        self.in_place(plaintext_and_ciphertext);
    }

    #[inline]
    pub fn decrypt_slice(&mut self, ciphertext_and_plaintext: &mut [u8]) {
        self.in_place(ciphertext_and_plaintext);
    }
}

impl Clone for XorCipher {
    fn clone(&self) -> Self {
        XorCipher {
            key: self.key.clone(),
        }
    }
}