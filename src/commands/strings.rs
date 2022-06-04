//  Library
use crate::lib::strings;
use clap::Args;
use std::str::FromStr;

// ======
// STRING
// ======

/// Generate a random string
///
/// Generate a string of random characters containing alphanumerics and special characters
#[derive(Args)]
#[clap(verbatim_doc_comment)]
pub struct Strings {
    /// Length of the string to generate
    #[clap(short, long, default_value_t = 16)]
    length: u8,

    #[clap(short, long, default_value = "all")]
    charset: strings::Charset,

    /// Number of times to repeat command execution
    #[clap(short, long, default_value_t = 1)]
    repeat: u8,
}

impl Strings {
    pub fn execute(&self) {
        let mut result: Vec<String> = Vec::new(); //  Vector to store results

        //  Generate random strings and store in results
        for _ in 00..self.repeat {
            let s: String = strings::generate_random(&self.charset, self.length as usize);
            result.push(s);
        }

        //  Show results
        println!("{}", result.join("\n"));
    }
}

//  =========
//  UTILITIES
//  =========

/// Implement FromStr trait for Charset to allow mapping
///  command-line argument strings to the Charset enum
impl FromStr for strings::Charset {
    type Err = clap::Error;     //? Handle the error properly

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "lowercase" => Ok(Self::LowercaseAlphabets),
            "uppercase" => Ok(Self::UppercaseAlphabets),
            "alphabets" => Ok(Self::Alphabets),
            "numbers" => Ok(Self::Numbers),
            "alphanumeric" => Ok(Self::Alphanumeric),
            "special" => Ok(Self::Special),
            "all" => Ok(Self::All),
            _ => Ok(Self::All),     //? Should let the user know that this was an invalid charset
        }
    }
}
