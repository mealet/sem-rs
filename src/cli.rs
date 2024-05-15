use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name="sem",
    long_about = "Encryption and Decryption by token string"
)]
pub struct Args {
    #[clap(
        short = 'i',
        long = "input",
        help = "Input for encrypt",
        default_value = "none",
        display_order = 1
    )]
    pub input: String,

    #[clap(
        short = 'o',
        long = "option",
        help = "Program option. 1 - encrypt, 2 - decrypt",
        default_value_t = 1,
        value_name = "number",
        display_order = 2,
    )]
    pub opt: u8,

    #[clap(
        short = 't',
        long = "token",
        help = "Custom token for encryption (required for decrypt)",
        required = false,
        default_value = "none",
        display_order = 3
    )]
    pub custom_token: String
}