use std::string::FromUtf8Error;

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
                            let idx = (buf >> (18 - i * 6)) & 0b111111;
                            BASE64_ALPHABET.chars().nth(idx as usize).unwrap()
                        } else {
                            '='
                        }
                    })
                    .collect::<String>()
            })
            .collect()
    }

    pub fn decode(&self, input: &str) -> Result<String, Base64Error> {
        let mut bytes: Vec<u8> = Vec::new();
        let mut buf: u32 = 0;
        let mut padding_count = 0;

        for (i, ch) in input.chars().enumerate() {
            let n = i % 4;
            match ch {
                '=' => {
                    padding_count += 1;
                }
                _ => {
                    let idx: u32 = BASE64_ALPHABET
                        .find(ch)
                        .ok_or(Base64Error::InvalidCharacter)?
                        as u32;
                    buf |= (idx << (18 - n * 6)) & 0xFFFFFF;
                }
            }

            if n == 3 {
                (0..3 - padding_count).for_each(|i| {
                    let byte = (buf >> (16 - i * 8)) as u8;
                    bytes.push(byte);
                });
                buf = 0;
            }
        }

        String::from_utf8(bytes).map_err(Base64Error::Utf8Error)
    }
}

#[derive(Debug)]
pub enum Base64Error {
    Utf8Error(FromUtf8Error),
    InvalidCharacter,
}

impl std::fmt::Display for Base64Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Base64Error::InvalidCharacter => write!(f, "Invalid character in input"),
            Base64Error::Utf8Error(ref e) => e.fmt(f),
        }
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

    #[test]
    fn test_base64_decoder() {
        let base64 = Base64::new();
        assert_eq!(base64.decode("").unwrap(), "");
        assert_eq!(base64.decode("Zg==").unwrap(), "f");
        assert_eq!(base64.decode("Zm8=").unwrap(), "fo");
        assert_eq!(base64.decode("Zm9v").unwrap(), "foo");
        assert_eq!(base64.decode("Zm9vYg").unwrap(), "foo");
        assert_eq!(base64.decode("Zm9vYg==").unwrap(), "foob");
        assert_eq!(base64.decode("Zm9vYmE=").unwrap(), "fooba");
        assert_eq!(base64.decode("Zm9vYmFy").unwrap(), "foobar");
    }
}
