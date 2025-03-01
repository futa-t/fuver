use serde::{Deserialize, Serialize};
use std::{fmt, result};

use crate::identifier;

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
        write!(f, "{}", &self.tag)?;
        if let Some(n) = self.number {
            write!(f, ".{}", n)?;
        }
        Ok(())
    }
}

pub type Result<T> = result::Result<T, PreReleaseError>;

#[derive(Debug)]
pub enum PreReleaseError {
    Undefined,
    Format(String),
    InvalidNumber(usize),
    Overflow(usize),
}

impl fmt::Display for PreReleaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PreReleaseError::Format(s) => write!(f, "フォーマットが不正です: {}", s),
            PreReleaseError::InvalidNumber(n) => write!(f, "数値に{}は指定できません", n),
            PreReleaseError::Overflow(n) => {
                write!(f, "数値が指定できる範囲を超えています: {}+1", n)
            }
            PreReleaseError::Undefined => write!(f, "プレリリースが定義されていません"),
        }
    }
}

fn check_identifier(tag: &str) -> Result<()> {
    identifier::check_identifier(tag).map_err(|e| PreReleaseError::Format(e.to_string()))
}

fn check_number_identifier(n: usize) -> Result<()> {
    if n == 0 {
        return Err(PreReleaseError::InvalidNumber(n));
    }
    Ok(())
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
    pub fn new(tag: &str) -> Result<PreRelease> {
        check_identifier(tag)?;
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
    pub fn with_number(tag: &str, number: usize) -> Result<PreRelease> {
        check_identifier(tag)?;
        check_number_identifier(number)?;
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
    pub fn set_tag(&mut self, tag: &str) -> Result<()> {
        check_identifier(tag)?;
        self.tag = tag.to_string();
        Ok(())
    }

    /// set new number
    ///
    /// # Errors
    /// number is 0
    pub fn set_number(&mut self, number: usize) -> Result<()> {
        check_number_identifier(number)?;
        self.number = Some(number);
        Ok(())
    }

    /// increment number
    ///
    /// If number is None. initialize 1.
    pub fn increment_number(&mut self) -> Result<()> {
        let mut new_n = self.number.unwrap_or(0);
        new_n = new_n
            .checked_add(1)
            .ok_or(PreReleaseError::Overflow(new_n))?;
        self.number = Some(new_n);
        Ok(())
    }

    /// set tag and number
    ///
    /// # Errors
    /// tag empty or invalid character included
    /// number is 0
    pub fn set(&mut self, tag: &str, number: usize) -> Result<()> {
        check_identifier(tag)?;
        check_number_identifier(number)?;
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
    fn test_default_prerelease() {
        let pre = PreRelease::default();
        assert_eq!(pre.tag, "alpha");
        assert_eq!(pre.number, None);
        assert_eq!(pre.to_string(), "alpha");
    }

    #[test]
    fn test_new_prerelease() {
        let pre = PreRelease::new("beta").unwrap();
        assert_eq!(pre.tag, "beta");
        assert_eq!(pre.number, None);
        assert_eq!(pre.to_string(), "beta");
    }

    #[test]
    fn test_with_number() {
        let pre = PreRelease::with_number("rc", 1).unwrap();
        assert_eq!(pre.tag, "rc");
        assert_eq!(pre.number, Some(1));
        assert_eq!(pre.to_string(), "rc.1");
    }

    #[test]
    fn test_invalid_number() {
        let result = PreRelease::with_number("beta", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_methods() {
        let pre = PreRelease::with_number("beta", 2).unwrap();
        assert_eq!(pre.get_tag(), "beta");
        assert_eq!(pre.get_number(), Some(2));
    }

    #[test]
    fn test_set_tag() {
        let mut pre = PreRelease::default();
        pre.set_tag("beta").unwrap();
        assert_eq!(pre.tag, "beta");
        assert_eq!(pre.to_string(), "beta");
    }

    #[test]
    fn test_set_invalid_tag() {
        let mut pre = PreRelease::default();
        let result = pre.set_tag("");
        assert!(result.is_err());

        assert_eq!(pre.tag, "alpha");
    }

    #[test]
    fn test_set_number() {
        let mut pre = PreRelease::default();
        pre.set_number(3).unwrap();
        assert_eq!(pre.number, Some(3));
        assert_eq!(pre.to_string(), "alpha.3");
    }

    #[test]
    fn test_set_invalid_number() {
        let mut pre = PreRelease::default();
        let result = pre.set_number(0);
        match result {
            Err(PreReleaseError::InvalidNumber(n)) => assert_eq!(n, 0),
            _ => panic!("Expected InvalidNumber error"),
        }

        assert_eq!(pre.number, None);
    }

    #[test]
    fn test_increment_number_from_none() {
        let mut pre = PreRelease::new("beta").unwrap();
        assert_eq!(pre.number, None);

        pre.increment_number().unwrap();
        assert_eq!(pre.number, Some(1));
        assert_eq!(pre.to_string(), "beta.1");
    }

    #[test]
    fn test_increment_number() {
        let mut pre = PreRelease::with_number("rc", 1).unwrap();
        pre.increment_number().unwrap();
        assert_eq!(pre.number, Some(2));
        assert_eq!(pre.to_string(), "rc.2");
    }

    #[test]
    fn test_increment_number_overflow() {
        let mut pre = PreRelease::with_number("rc", usize::MAX).unwrap();
        let result = pre.increment_number();

        match result {
            Err(PreReleaseError::Overflow(n)) => assert_eq!(n, usize::MAX),
            _ => panic!("Expected Overflow error"),
        }

        assert_eq!(pre.number, Some(usize::MAX));
    }

    #[test]
    fn test_set() {
        let mut pre = PreRelease::default();
        pre.set("rc", 2).unwrap();
        assert_eq!(pre.tag, "rc");
        assert_eq!(pre.number, Some(2));
        assert_eq!(pre.to_string(), "rc.2");
    }

    #[test]
    fn test_set_invalid() {
        let mut pre = PreRelease::default();

        let result = pre.set("", 1);
        assert!(result.is_err());

        let result = pre.set("beta", 0);
        assert!(result.is_err());

        assert_eq!(pre.tag, "alpha");
        assert_eq!(pre.number, None);
    }

    #[test]
    fn test_fmt_display() {
        // タグのみ
        let pre = PreRelease::new("alpha").unwrap();
        assert_eq!(format!("{}", pre), "alpha");

        let pre = PreRelease::with_number("beta", 2).unwrap();
        assert_eq!(format!("{}", pre), "beta.2");
    }

    #[test]
    fn test_error_display() {
        let err = PreReleaseError::Format("test".to_string());
        assert_eq!(format!("{}", err), "フォーマットが不正です: test");

        let err = PreReleaseError::InvalidNumber(0);
        assert_eq!(format!("{}", err), "数値に0は指定できません");

        // Overflow error
        let err = PreReleaseError::Overflow(100);
        assert_eq!(
            format!("{}", err),
            "数値が指定できる範囲を超えています: 100+1"
        );
    }
}
