const BASE64_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub struct Base64;

impl Base64 {
    pub fn new() -> Self {
        Self {}
    }

    pub fn encode(&self, input: &str) -> String {
        let bytes = input.as_bytes();

        bytes
            .chunks(3)
            .map(|chunk| {
                // Create a 24-bit buffer
                let mut buf: u32 = 0;
                for (i, &byte) in chunk.iter().enumerate() {
                    buf |= (byte as u32) << (16 - i * 8);
                }

                // Encode the 24-bit buffer into 4 Base64 characters
                (0..4)
                    .map(|i| {
                        if i < chunk.len() + 1 {
                            let idx = (buf >> 18 - i * 6) & 0b111111;
                            BASE64_ALPHABET.chars().nth(idx as usize).unwrap()
                        } else {
                            '='
                        }
                    })
                    .collect::<String>()
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::Base64;

    #[test]
    fn test_base64_encoder() {
        let base64 = Base64::new();
        assert_eq!(base64.encode(""), "");
        assert_eq!(base64.encode("f"), "Zg==".to_string());
        assert_eq!(base64.encode("fo"), "Zm8=".to_string());
        assert_eq!(base64.encode("foo"), "Zm9v".to_string());
        assert_eq!(base64.encode("foob"), "Zm9vYg==".to_string());
        assert_eq!(base64.encode("fooba"), "Zm9vYmE=".to_string());
        assert_eq!(base64.encode("foobar"), "Zm9vYmFy".to_string());
    }
}
