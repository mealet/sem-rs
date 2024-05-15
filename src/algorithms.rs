const ALPHABET: [char; 57] = [' ', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    '!', '?', '.', ',', '/', '(', ')', '[', ']', '@', '#', '$', '%', '^', '&', '*', '-', '_', '+', '=', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

pub fn encrypt(input: String, token_string: String) -> String {
    let token = token_string.chars().collect::<Vec<char>>();
    let mut output = String::new();

    for c in input.chars() {
        let ALPHABET_position = ALPHABET.iter().position(|r| *r == c);
        match ALPHABET_position {
            Some(t) => {
                output += &token[t].to_string();
            },
            None => {}
        };
    }

    return output;
}

pub fn decrypt(input: String, token_string: String) -> String {
    let token = token_string.chars().collect::<Vec<char>>();
    let mut output = String::new();

    for c in input.chars() {
        let token_position = &token.iter().position(|r| *r == c);
        match token_position {
            Some(t) => {
                output += ALPHABET[*t].to_string().as_str();
            },
            None => {}
        };
    }

    return output;
}

#[cfg(test)]
mod tests {
    use crate::functions::generate_token;
    use super::*;

    #[test]
    fn test_encryption() {
        let encrypted = encrypt("hello world".to_string(), "n01t]W;6sDRbX?d}x-KG~&BJoA9".to_string());

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