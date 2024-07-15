// String Encryption Method aka SEM
// https://github.com/mealet/sem-rs
// --------------------------------
// Project licensed under the MIT License.
// More in the LICENSE file

use rand::{seq::SliceRandom, thread_rng};
use std::collections::HashSet;

use crate::{functions, TOKEN_LEN};

pub fn check_arg(argument: String) -> Option<String> {
    let none_stroke = "none".to_string();
    let output;

    if argument == none_stroke || argument.len() < 1 || argument.replace(" ", "").len() < 1 {
        output = None;
    } else {
        output = Some(argument);
    }

    return output;
}

pub fn generate_token(length: usize) -> Result<String, String> {
    let chars: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()-_=+[]{};:',.<>?|`~".chars().collect();

    if length > chars.len() {
        return Err("Too much length".to_string());
    }

    let mut rng = thread_rng();
    let mut shuffled_chars = chars.clone();
    shuffled_chars.shuffle(&mut rng);

    let random_string: String = shuffled_chars.iter().take(length).collect();

    return Ok(random_string);
}

pub fn remove_duplicates(input: &String) -> String {
    let mut seen = HashSet::new();
    input.chars().filter(|&c| seen.insert(c)).collect()
}

pub fn check_token(token: String) -> Result<(), String> {
    let formatted_token = token.replace(format!("{}", crate::algorithms::DEVCODE).as_str(), "");
    if formatted_token.len() < TOKEN_LEN {
        return Err(format!(
            "TOKEN's length must be at least {} symbols",
            TOKEN_LEN
        ));
    }
    if functions::remove_duplicates(&formatted_token).len() < formatted_token.len() {
        return Err("TOKEN contains duplicates! Remove it and repeat".to_string());
    }

    return Ok(());
}
