use std::string::FromUtf8Error;

/// The Base64 alphabet, used for encoding and decoding Base64 strings.
const BASE64_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// The `Base64` struct provides methods for encoding and decoding strings in Base64 format.
pub struct Base64;

impl Base64 {
    /// Creates a new `Base64` encoder/decoder instance.
    ///
    /// # Example
    ///
    /// ```
    /// let base64 = Base64::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }

    /// Encodes the given input string into a Base64-encoded string.
    ///
    /// This method processes the input string as bytes, groups them in chunks of 3, and converts
    /// each chunk into a 24-bit buffer. The buffer is then split into four 6-bit groups, which
    /// are mapped to Base64 characters. Padding is added as necessary.
    /// read more: https://datatracker.ietf.org/doc/html/rfc4648
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that will be encoded.
    ///
    /// # Returns
    ///
    /// A `String` containing the Base64-encoded result.
    ///
    /// # Example
    ///
    /// ```
    /// let base64 = Base64::new();
    /// let encoded = base64.encode("Hello");
    /// assert_eq!(encoded, "SGVsbG8=");
    /// ```
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

    /// Decodes a Base64-encoded string into its original plain text form.
    ///
    /// This method processes the input string in groups of four Base64 characters at a time.
    /// Each 4-character group corresponds to a 24-bit block, which is then split back into its
    /// original 3-byte (24-bit) form. Padding characters (`=`) are correctly managed to ensure
    /// the output has the correct byte length, and any remaining bytes are properly decoded.
    /// read more: https://datatracker.ietf.org/doc/html/rfc4648
    ///
    /// If the input contains an invalid character not present in the Base64 alphabet, a
    /// `Base64Error::InvalidCharacter` error is returned. If the decoded byte sequence is not
    /// valid UTF-8, the method returns a `Base64Error::Utf8Error`.
    ///
    /// # Errors
    ///
    /// - `Base64Error::InvalidCharacter`: Returned when the input contains characters outside
    ///   the standard Base64 alphabet.
    /// - `Base64Error::Utf8Error`: Returned when the decoded byte sequence cannot be converted
    ///   into a valid UTF-8 string.
    ///
    /// # Example
    ///
    /// ```
    /// let base64 = Base64::new();
    /// let decoded = base64.decode("SGVsbG8=").unwrap();
    /// assert_eq!(decoded, "Hello");
    /// ```

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

/// Custom error type for Base64 encoding/decoding operations.
#[derive(Debug)]
pub enum Base64Error {
    /// Error returned when the decoded bytes cannot be converted to a valid UTF-8 string.
    Utf8Error(FromUtf8Error),
    /// Error returned when an invalid Base64 character is encountered during decoding.
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
