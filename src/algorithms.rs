// String Encryption Method aka SEM
// https://github.com/mealet/sem-rs
// --------------------------------
// Project licensed under the MIT License.
// More in the LICENSE file/ String Encryption Method aka SEM
// https://github.com/mealet/sem-rs
// --------------------------------
// Project licensed under the MIT License.
// More in the LICENSE file

use chrono::prelude::*;

const ALPHABET: [char; 64] = [
    ' ', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '!', '?', '.', ',', '/', '(', ')', '[', ']', '@', '#',
    '$', '%', '^', '&', '*', '-', '_', '+', '=', '`', '~', '"', '1', '2', '3', '4', '5', '6', '7',
    '8', '9', '0', ':', ';', '\\', '\n',
];

pub const DEVCODE: &str = "0x7D9:";

pub fn encrypt(input: String, token_string: String) -> String {
    let token = token_string.chars().collect::<Vec<char>>();
    let mut output = String::new();

    let formatted_input = format!(
        "{}--[timestamp:{}]",
        input.clone().trim(),
        Utc::now().timestamp()
    );

    for c in formatted_input.chars() {
        let alphabet_position = ALPHABET.iter().position(|r| *r == c);
        match alphabet_position {
            Some(t) => {
                output += &token[t].to_string();
            }
            None => {}
        };
    }

    let reversed_output = output.chars().rev().collect::<String>();

    return reversed_output;
}

pub fn decrypt(input: String, token_string: String) -> String {
    let mut token = token_string.clone();
    let mut output = String::new();

    let mut dev_mode = false;

    if token.starts_with(DEVCODE) {
        dev_mode = true;
        token = token.replace(format!("{}", DEVCODE).as_str(), "");
    }

    let token_chars = token.chars().collect::<Vec<char>>();
    let reversed_input = input.trim().chars().rev().collect::<String>();

    for c in reversed_input.chars() {
        let token_position = &token_chars.iter().position(|r| *r == c);
        match token_position {
            Some(t) => {
                output += ALPHABET[*t].to_string().as_str();
            }
            None => {}
        };
    }

    if !dev_mode {
        output = output.split("--[timestamp:").next().unwrap().to_string();
    }

    return output;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::functions::generate_token;

    #[test]
    fn test_encryption() {
        let encrypted = encrypt(
            "hello world".to_string(),
            "n01t]W;6sDRbX?d}x-KG~&BJoA9".to_string(),
        );

        println!("{}", encrypted);

        assert_eq!(encrypted, "sWXX}nJ}KX]");
    }

    #[test]
    fn test_decryption() {
        let encrypted = "sWXX}nJ}KX]";
        let token = "n01t]W;6sDRbX?d}x-KG~&BJoA9";

        let decrypted = decrypt(encrypted.to_string(), token.to_string());
        println!("{}", decrypted.clone());

        assert_eq!(decrypted, "hello world");
    }

    #[test]
    fn real_test() {
        let tok = generate_token(27).unwrap();
        let enc = encrypt("hello world".to_string(), tok.clone());

        println!("{}  ||  {}", tok, enc);

        let dec = decrypt(enc, tok);
        println!("{}", dec.clone());

        assert_eq!(dec, "hello world");
    }
}
