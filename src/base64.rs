const BASE64_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub struct Base64;

impl Base64 {
    pub fn new() -> Self {
        Self {}
    }

    pub fn encode(self, input: &str) -> String {
        let bytes = input.as_bytes();

        bytes
            .chunks(3)
            .map(|chunk| {
                let mut buf: u32 = 0;

                (0..3).for_each(|i| {
                    let b = chunk.get(i as usize).unwrap_or(&0);
                    buf = (buf << 8) | *b as u32;
                });

                let mut encoded_chunk = "".to_string();
                for i in 0..4 {
                    if i < chunk.len() + 1 {
                        let idx = (buf >> 18 - i * 6) & 0b111111;
                        let base64_char = BASE64_ALPHABET.chars().nth(idx as usize).unwrap();
                        encoded_chunk.push(base64_char);
                    } else {
                        encoded_chunk.push('=');
                    }
                }
                encoded_chunk
            })
            .collect::<String>()
    }
}
