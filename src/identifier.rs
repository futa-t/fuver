use std::fmt;

#[derive(Debug)]
pub enum FormatError {
    EmptyTag,
    InvalidChar(char),
    InvalidNumber,
}

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormatError::EmptyTag => write!(f, "空文字列は対応していません"),
            FormatError::InvalidChar(c) => {
                write!(f, "対応していない文字が含まれています: {}", c)
            }
            FormatError::InvalidNumber => write!(f, "0は指定できません"),
        }
    }
}

pub fn check_tag(s: &str) -> Result<(), FormatError> {
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
