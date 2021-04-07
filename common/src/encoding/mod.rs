pub mod charmap;

use std::error::Error;
use std::fmt::{self, Display, Formatter};

use charmap::{Encoding, CHARMAP};

#[derive(Debug)]
pub struct EncodingError {
    pub valid_string: String,
    pub invalid: u8,
    pub error_index: usize,
}

impl Display for EncodingError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Invalid character 0x{:02X} at position {}",
            self.invalid, self.error_index
        )
    }
}

pub fn parse_string(raw: &[u8]) -> Result<String, EncodingError> {
    enum InternalError {
        Encoding(EncodingError),
        // Use an error variant for control flow so that we can short-circuit with try_fold
        Stop(String),
    }

    let final_state = raw.iter().enumerate().try_fold(
        String::with_capacity(raw.len()),
        |mut acc, (i, raw_char)| match CHARMAP[*raw_char as usize] {
            Encoding::Char(c) => {
                acc.push(c);
                Ok(acc)
            }
            Encoding::Invalid => Err(InternalError::Encoding(EncodingError {
                valid_string: acc,
                invalid: *raw_char,
                error_index: i,
            })),
            Encoding::End => Err(InternalError::Stop(acc)),
        },
    );
    match final_state {
        Ok(s) | Err(InternalError::Stop(s)) => Ok(s),
        Err(InternalError::Encoding(e)) => Err(e),
    }
}

pub fn parse_string_lossy(mut raw: &[u8]) -> String {
    let mut result = String::with_capacity(raw.len());
    while !raw.is_empty() {
        match parse_string(raw) {
            Ok(valid) => {
                result.push_str(&valid);
                break;
            }
            Err(e) => {
                result.push_str(&e.valid_string);
                result.push('\u{FFFD}');
                if e.error_index + 1 < raw.len() {
                    raw = &raw[e.error_index + 1..];
                } else {
                    break;
                }
            }
        }
    }
    result
}
