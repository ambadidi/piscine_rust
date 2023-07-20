use std::u32;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    // expected public fields
    pub validation: bool,
    pub expected: String,
}
impl CipherError {
    #[allow(dead_code)]
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError {
            validation,
            expected,
        }
    }
}
pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if original.trim().is_empty() || ciphered.trim().is_empty() {
        return None;
    }
    let mut correct = String::new();
    for ch in original.chars() {
        if ch.is_alphabetic() && ch.is_lowercase() {
            let letter =
                char::from_u32(((u32::from(ch) - u32::from('a')) * 25 + 25) % 26 + u32::from('a'))
                    .unwrap();
            correct.push(letter);
        } else if ch.is_alphabetic() {
            let letter =
                char::from_u32(((u32::from(ch) - u32::from('A')) * 25 + 25) % 26 + u32::from('A'))
                    .unwrap();
            correct.push(letter);
        } else {
            correct.push(ch);
        }
    }
    if correct != ciphered {
        return Some(Err(CipherError {
            validation: false,
            expected: correct,
        }));
    }
    Some(Ok(true))
}
