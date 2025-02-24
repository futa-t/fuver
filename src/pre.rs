use serde::{Deserialize, Serialize};
use std::{fmt, result};

use crate::identifier::{check_tag, FormatError};

pub type Result = result::Result<(), FormatError>;

/// Pre-release version information for semantic versioning
///
/// consisting of a tag and an optional number (e.g., "alpha.1", "beta.2").
#[derive(Serialize, Deserialize, Clone)]
pub struct PreRelease {
    tag: String,
    number: Option<usize>,
}

impl fmt::Display for PreRelease {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "-{}", &self.tag)?;
        if let Some(n) = self.number {
            write!(f, ".{}", n)?;
        }
        Ok(())
    }
}

impl PreRelease {
    /// Creates a new PreRelease with tag.
    ///
    /// # Arguments
    /// * `tag` - pre-release tag
    ///
    /// # Errors
    /// Returns `FormatError`
    /// * tag is empty
    /// * tag contains invalid characters
    ///
    /// # Examples
    /// ```
    /// let pre = PreRelease::new("alpha").unwrap();
    /// assert_eq!(pre.to_string(), "-alpha");
    /// ```
    pub fn new(tag: &str) -> result::Result<PreRelease, FormatError> {
        check_tag(tag)?;
        Ok(PreRelease {
            tag: tag.to_string(),
            number: None,
        })
    }

    /// Creates a new PreRelease with tag and number.
    ///
    /// # Errors
    /// * tag empty or invalid character included
    /// * number is 0
    pub fn with_number(tag: &str, number: usize) -> result::Result<PreRelease, FormatError> {
        check_tag(tag)?;
        Ok(PreRelease {
            tag: tag.to_string(),
            number: Some(number),
        })
    }

    pub fn get_tag(&self) -> String {
        self.tag.to_string()
    }

    pub fn get_number(&self) -> Option<usize> {
        self.number
    }

    /// set new tag
    ///
    /// # Errors
    /// empty or invalid character included
    pub fn set_tag(&mut self, tag: &str) -> Result {
        check_tag(tag)?;
        self.tag = tag.to_string();
        Ok(())
    }

    /// set new number
    ///
    /// # Errors
    /// number is 0
    pub fn set_number(&mut self, number: usize) -> Result {
        if number == 0 {
            return Err(FormatError::InvalidNumber);
        }
        self.number = Some(number);
        Ok(())
    }

    /// set tag and number
    ///
    /// # Errors
    /// tag empty or invalid character included
    /// number is 0
    pub fn set(&mut self, tag: &str, number: usize) -> Result {
        self.set_tag(tag)?;
        self.set_number(number)?;
        Ok(())
    }

    /// print pre-release info
    pub fn show(&self) {
        println!("{}", self);
    }
}

impl Default for PreRelease {
    /// Returns the default PreRelease.
    /// tag "alpha" and no number.
    fn default() -> Self {
        PreRelease::new("alpha").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(PreRelease::new("alpha").is_ok());
        assert!(PreRelease::new("").is_err());
        assert!(PreRelease::new("alpha&").is_err());
    }

    #[test]
    fn test_with_number() {
        let pre = PreRelease::with_number("beta", 1).unwrap();
        assert_eq!(pre.get_tag(), "beta");
        assert_eq!(pre.get_number(), Some(1));
    }

    #[test]
    fn test_display() {
        let pre = PreRelease::with_number("rc", 2).unwrap();
        assert_eq!(pre.to_string(), "-rc.2");
    }
}
