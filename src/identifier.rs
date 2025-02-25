use std::{fmt, result};

pub type Result<T> = result::Result<T, FormatError>;

#[derive(Debug)]
pub enum FormatError {
    EmptyTag,
    InvalidChar(char),
}

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormatError::EmptyTag => write!(f, "空文字列は対応していません"),
            FormatError::InvalidChar(c) => {
                write!(f, "対応していない文字が含まれています: {}", c)
            }
        }
    }
}

pub fn check_dot_separated_identifiers(s: &str) -> Result<()> {
    if s.is_empty() {
        return Err(FormatError::EmptyTag);
    }
    for part in s.split(".") {
        check_identifier(part)?
    }
    Ok(())
}

pub fn check_identifier(s: &str) -> Result<()> {
    if s.is_empty() {
        return Err(FormatError::EmptyTag);
    }

    match s
        .bytes()
        .find(|&b| !matches!(b, b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'-'))
    {
        None => Ok(()),
        Some(b) => Err(FormatError::InvalidChar(b as char)),
    }
}
