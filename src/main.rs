// English Alphabet Matches
// a b c d e f g h i j  k  l  m  n  o  p  q  r  s  t  u  v  w  x  y  z
// 1 2 3 4 5 6 7 8 9 10 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27
// Space is 0

// Algorithm:
// "some stroke" -> "/*\%^10*58vrdf"
// replacing chars with token and adding some salt like this: ".dj106hn"
// salt will between "-+-+-"
// so token will be like this: "8g0421g6?//5468%&&!*^(&-+-+-.dj106hn-+-+-"
// chars in token will be matched by number to alphabet

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const TOKEN_LEN: usize = 60;

use clap::Parser;
use colored::*;
use std::io::{stdin, stdout, Write};
use std::process::exit;

use crate::cli::Args;
use crate::functions::check_arg;

mod algorithms;
mod cli;
mod functions;

fn main() {
    // Parsing arguments

    let args = Args::parse();

    let greeting_message = format!(
        "{}\n{}",
        format!("{} - v{}", APP_NAME, APP_VERSION).green(),
        "WARNING: Encrypted string will be converted to lowercase".yellow(),
    );

    println!("{}", greeting_message);

    let mut args_opt = args.opt;
    let mut input = args.input;
    let mut custom_token = functions::check_arg(args.custom_token);

    let input_mode = match functions::check_arg(input.clone()) {
        Some(_) => false,
        None => true,
    };

    args_opt = match args.opt {
        0 => 1,
        1 => 1,
        2 => 2,
        _ => 1,
    };

    // Input Mode

    if input_mode {
        let mut imode_mode = 0;
        let mut imode_input = String::new();
        let mut imode_token;

        // Requesting mode

        let mut temp = String::new();

        print!("Choose your mode (1 - encrypt, 2 - decrypt): ");
        let _ = stdout().flush();
        let _ = stdin().read_line(&mut temp);

        imode_mode = temp.trim().parse().unwrap_or_else(|e| {
            println!(
                "{}",
                "WARNING: Mode parsing error! Changing mode to default 1 (encrypt)"
            );
            1
        });

        imode_mode = match imode_mode {
            1 => 1,
            2 => 2,
            _ => 1,
        };

        drop(temp);

        loop {
            print!("Type input: ");
            let _ = stdout().flush();
            let _ = stdin().read_line(&mut imode_input);

            if imode_input.clone().trim().replace(" ", "").len() < 1 {
                eprintln!("{}", "Type something to input!".red());
            } else {
                break;
            }
        }

        match imode_mode {
            1 => print!("Type custom token (you can leave that empty for auto-generate): "),
            2 => print!("Type decrypt token: "),
            _ => print!("Type custom token (you can leave that empty for auto-generate): "),
        };
        let mut temp_token = String::new();
        let _ = stdout().flush();
        let _ = stdin().read_line(&mut temp_token);

        imode_token = functions::check_arg(temp_token.trim().replace(" ", ""));

        input = imode_input;
        custom_token = imode_token;
        args_opt = imode_mode;
    }

    // Greeting

    let opt = match args_opt {
        1 => "encrypt",
        2 => "decrypt",
        _ => "error",
    };

    // Main Process

    match args_opt {
        1 => {
            // Encryption

            let mut output = String::new();

            input = input.to_lowercase();

            if let Some(token) = custom_token {
                match functions::check_token(token.clone()) {
                    Ok(_) => {
                        output = algorithms::encrypt(input, token.clone());
                    }
                    Err(e) => {
                        eprintln!("{}", e.red());
                        exit(0);
                    }
                }

                println!(
                    "\n{} {}",
                    "Encryption successful! Here's the result:".green(),
                    output
                );
                println!("{} {}", "Your decryption token:".green(), token);
            } else {
                let generated_token = functions::generate_token(TOKEN_LEN + 2);

                if let Ok(token) = generated_token {
                    output = algorithms::encrypt(input, token.clone());

                    println!(
                        "\n{} {}",
                        "Encryption successful! Here's the result:".green(),
                        output
                    );
                    println!("{} {}", "Your decryption token:".green(), token);
                } else {
                    eprintln!("{}",
                        format!("Error with token auto-generate. Please write your custom token by '-t' argument.\nToken's length must be {} symbols", TOKEN_LEN).red()
                    )
                }
            }
        }
        2 => {
            // Decryption

            let mut output = String::new();

            if let Some(token) = custom_token {
                if let Ok(_) = functions::check_token(token.clone()) {
                    output = algorithms::decrypt(input.clone(), token.clone());

                    println!("\n{} {}", "Decrypted stroke: ".green(), output);
                } else if let Err(e) = functions::check_token(token.clone()) {
                    eprintln!("{}", e.red());
                    exit(0);
                }
            } else {
                let mut user_input = String::new();

                println!("{}", "No token found!".red());

                print!("Type decrypt token: ");
                let _ = stdout().flush();

                let _ = std::io::stdin().read_line(&mut user_input);
                let user_token = check_arg(user_input);

                match user_token {
                    Some(token) => {
                        if let Ok(_) = functions::check_token(token.clone()) {
                            output = algorithms::decrypt(input, token.clone());

                            println!("\n{} {}", "Decrypted stroke: ".green(), output);
                        } else if let Err(e) = functions::check_token(token.clone()) {
                            eprintln!("{}", e.red());
                            exit(0);
                        }
                    }
                    None => {
                        eprintln!(
                            "{}",
                            "Decryption requires token! Restart program and try again".red()
                        );
                        exit(0)
                    }
                }
            }
        }
        _ => {
            eprintln!("{}", "An error occured with parsing option".red());
            exit(0)
        }
    }
}
