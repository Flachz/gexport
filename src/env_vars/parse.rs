use super::EnvironmentVariable;
use std::ffi::{OsStr, OsString};
use std::error::Error;
use std::fmt::Display;
use std::fmt;
use std::env;
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use regex::Regex;

impl EnvironmentVariable {
    pub(crate) fn new(name: Vec<u8>) -> Result<Self, ParserError> {
        let name = match String::from_utf8(name) {
            Ok(name) => name,
            Err(error) => {
                return Err(ParserError::InvalidIdentifier(
                    String::from_utf8_lossy(error.as_bytes()).to_string())
                );
            }
        };

        if !Self::is_identifier_valid(&name) {
            Err(ParserError::InvalidIdentifier(name))
        } else {
            Ok(Self {
                name,
                value: Vec::default(),
            })
        }
    }
    
    pub(crate) fn from_parse(input: &OsStr) -> Result<Self, ParserError> {
        let input = input.as_bytes();
        let divider = input
            .iter()
            .position(|&byte| byte == '=' as u8);
        
        let (name, value) = if let Some(divider) = divider {
            (&input[..divider], Some(input[divider + 1..].to_vec()))
        } else {
            (input, None)
        };

        let mut this = Self::new(name.to_vec())?;

        let value = match value {
            Some(value) => value,
            None => {
                env::var_os(&this.name)
                    .unwrap_or(OsString::default())
                    .into_vec()
            }
        };

        this.value = value;
        
        Ok(this)
    }

    pub(crate) fn is_identifier_valid(name: &str) -> bool {
        let valid_name = Regex::new("^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
        valid_name.is_match(name)
    }
}

#[derive(Debug)]
pub(crate) enum ParserError {
    InvalidIdentifier(String),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier(name) => write!(f, "'{name}' is not a valid identifier"),
        }
    }
}

impl Error for ParserError {}