// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Translate to and from Morse code.
//!
//! See [Morse_code](https://en.wikipedia.org/wiki/Morse_code)

use std::collections::HashMap;
use std::sync::OnceLock;

// Exclamation mark is not in ITU-R recommendation
const MORSE_CODE_LIST: [(u8, &str); 52] = [
    (b'A', ".-"),
    (b'B', "-..."),
    (b'C', "-.-."),
    (b'D', "-.."),
    (b'E', "."),
    (b'F', "..-."),
    (b'G', "--."),
    (b'H', "...."),
    (b'I', ".."),
    (b'J', ".---"),
    (b'K', "-.-"),
    (b'L', ".-.."),
    (b'M', "--"),
    (b'N', "-."),
    (b'O', "---"),
    (b'P', ".--."),
    (b'Q', "--.-"),
    (b'R', ".-."),
    (b'S', "..."),
    (b'T', "-"),
    (b'U', "..-"),
    (b'V', "...-"),
    (b'W', ".--"),
    (b'X', "-..-"),
    (b'Y', "-.--"),
    (b'Z', "--.."),
    (b'1', ".----"),
    (b'2', "..---"),
    (b'3', "...--"),
    (b'4', "....-"),
    (b'5', "....."),
    (b'6', "-...."),
    (b'7', "--..."),
    (b'8', "---.."),
    (b'9', "----."),
    (b'0', "-----"),
    (b'&', ".-..."),
    (b'@', ".--.-."),
    (b':', "---..."),
    (b',', "--..--"),
    (b'.', ".-.-.-"),
    (b'\'', ".----."),
    (b'"', ".-..-."),
    (b'?', "..--.."),
    (b'/', "-..-."),
    (b'=', "-...-"),
    (b'+', ".-.-."),
    (b'-', "-....-"),
    (b'(', "-.--."),
    (b')', "-.--.-"),
    (b'!', "-.-.--"),
    (b' ', "/"),
];

const DELIMITER: &str = " ";

/// # Errors
///
/// Returns error if the character is not in morse code table.
pub fn encrypt(message: &str) -> Result<String, char> {
    static MORSE_CODE_DICT: OnceLock<HashMap<u8, &'static str>> = OnceLock::new();
    let map = MORSE_CODE_DICT.get_or_init(|| HashMap::from(MORSE_CODE_LIST));
    let message = message.to_ascii_uppercase();
    let mut out = String::new();
    for byte in message.as_bytes() {
        if let Some(s) = map.get(byte) {
            out.push_str(s);
            out.push_str(DELIMITER);
        } else {
            return Err(*byte as char);
        }
    }
    // Remove the last delimiter
    let _ = out.pop();
    Ok(out)
}

/// # Errors
///
/// Returns error if got invalid morse code from message.
pub fn decrypt(message: &str) -> Result<String, String> {
    static REVERSE_DICT: OnceLock<HashMap<&'static str, u8>> = OnceLock::new();
    let map = REVERSE_DICT.get_or_init(|| {
        MORSE_CODE_LIST
            .iter()
            .copied()
            .map(|(byte, s)| (s, byte))
            .collect()
    });

    let mut out = String::new();
    for part in message.split(DELIMITER) {
        if let Some(byte) = map.get(part) {
            out.push(*byte as char);
        } else {
            return Err(part.to_owned());
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::{decrypt, encrypt};

    #[test]
    fn test_encrypt() {
        assert_eq!(encrypt("Sos!").as_deref(), Ok("... --- ... -.-.--"));
        assert_eq!(encrypt("SOS!"), encrypt("sos!"));
    }

    #[test]
    fn test_decrypt() {
        assert_eq!(decrypt("... --- ... -.-.--").as_deref(), Ok("SOS!"));
    }
}
