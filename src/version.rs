use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use std::{fmt, ops::AddAssign, result, str::FromStr};

#[derive(Debug)]
pub enum VersionError {
    Format(String),
}

impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VersionError::Format(s) => write!(f, "バージョンが不正です: {}", s),
        }
    }
}
pub type Result<T> = result::Result<T, VersionError>;

fn part_version(version: &str) -> Result<Vec<usize>> {
    let sp: Vec<&str> = version.split(".").collect();
    if sp.iter().len() != 3 {
        return Err(VersionError::Format(version.to_string()));
    }

    Ok(sp.iter().map(|n| n.parse::<usize>().unwrap_or(0)).collect())
}

#[derive(Deserialize, Clone)]
pub struct Version {
    major: usize,
    minor: usize,
    patch: usize,
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Version", 4)?;
        state.serialize_field("major", &self.major)?;
        state.serialize_field("minor", &self.minor)?;
        state.serialize_field("patch", &self.patch)?;

        state.serialize_field("str", &self.to_string())?;

        state.end()
    }
}

impl FromStr for Version {
    type Err = VersionError;
    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let parts = part_version(s)?;
        Ok(Self {
            major: parts[0],
            minor: parts[1],
            patch: parts[2],
        })
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl AddAssign for Version {
    fn add_assign(&mut self, rhs: Self) {
        if rhs.major > 0 {
            self.major += rhs.major;
            self.minor = 0;
            self.patch = 0;
        } else if rhs.minor > 0 {
            self.minor += rhs.minor;
            self.patch = 0;
        } else {
            self.patch += rhs.patch;
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        Self {
            major: 0,
            minor: 1,
            patch: 0,
        }
    }
}

impl Version {
    pub fn new(major: usize, minor: usize, patch: usize) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn set_major(&mut self, n: usize) {
        self.major = n;
    }

    pub fn set_minor(&mut self, n: usize) {
        self.minor = n;
    }

    pub fn set_patch(&mut self, n: usize) {
        self.patch = n;
    }

    pub fn get_major(&self) -> usize {
        self.major
    }

    pub fn get_minor(&self) -> usize {
        self.minor
    }

    pub fn get_patch(&self) -> usize {
        self.patch
    }

    pub fn increment_major(&mut self) -> Result<()> {
        self.set_major(self.major + 1);
        self.minor = 0;
        self.patch = 0;
        Ok(())
    }

    pub fn increment_minor(&mut self) -> Result<()> {
        self.set_minor(self.minor + 1);
        self.patch = 0;
        Ok(())
    }

    pub fn increment_patch(&mut self) -> Result<()> {
        self.set_patch(self.patch + 1);
        Ok(())
    }

    /// Increment using mask
    ///
    /// Mask format is `x.x.x`. `x` can be any non-numeric.
    /// Change the part increment number.
    /// For example, if you increment the patch, specify the following `x.x.1`.
    /// if the version was `0.1.0`, become `0.1.1`.
    ///
    /// If you change the higher version, the lower version will be reset to 0. if you
    /// want to avoid this, use `set` method.
    ///
    /// The increment can be positive number. `x.2.x` will result `0.3.0`.
    pub fn increment_mask(&mut self, mask: &str) -> Result<()> {
        let mask_version = Version::from_str(mask)?;
        *self += mask_version;
        Ok(())
    }

    pub fn show_major(&self) -> Result<()> {
        println!("{}", self.get_major());
        Ok(())
    }

    pub fn show_minor(&self) -> Result<()> {
        println!("{}", self.get_minor());
        Ok(())
    }

    pub fn show_patch(&self) -> Result<()> {
        println!("{}", self.get_patch());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_default_version() {
        let version = Version::default();
        assert_eq!(version.major, 0);
        assert_eq!(version.minor, 1);
        assert_eq!(version.patch, 0);
        assert_eq!(version.to_string(), "0.1.0");
    }

    #[test]
    fn test_new_version() {
        let version = Version::new(1, 2, 3);
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
        assert_eq!(version.to_string(), "1.2.3");
    }

    #[test]
    fn test_from_str_valid() {
        let version = Version::from_str("2.3.4").unwrap();
        assert_eq!(version.major, 2);
        assert_eq!(version.minor, 3);
        assert_eq!(version.patch, 4);
    }

    #[test]
    fn test_from_str_invalid() {
        let result = Version::from_str("2.3");
        assert!(result.is_err());

        let result = Version::from_str("2.3.4.5");
        assert!(result.is_err());

        match Version::from_str("invalid") {
            Err(VersionError::Format(s)) => assert_eq!(s, "invalid"),
            _ => panic!("Expected VersionError::Format"),
        }
    }

    #[test]
    fn test_getters_and_setters() {
        let mut version = Version::new(1, 2, 3);

        version.set_major(4);
        assert_eq!(version.get_major(), 4);

        version.set_minor(5);
        assert_eq!(version.get_minor(), 5);

        version.set_patch(6);
        assert_eq!(version.get_patch(), 6);

        assert_eq!(version.to_string(), "4.5.6");
    }

    #[test]
    fn test_increment_major() {
        let mut version = Version::new(1, 2, 3);
        version.increment_major().unwrap();
        assert_eq!(version.major, 2);
        assert_eq!(version.minor, 0);
        assert_eq!(version.patch, 0);
        assert_eq!(version.to_string(), "2.0.0");
    }

    #[test]
    fn test_increment_minor() {
        let mut version = Version::new(1, 2, 3);
        version.increment_minor().unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 3);
        assert_eq!(version.patch, 0);
        assert_eq!(version.to_string(), "1.3.0");
    }

    #[test]
    fn test_increment_patch() {
        let mut version = Version::new(1, 2, 3);
        version.increment_patch().unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 4);
        assert_eq!(version.to_string(), "1.2.4");
    }

    #[test]
    fn test_add_assign() {
        let mut version = Version::new(1, 2, 3);

        // メジャーバージョンの増加
        let increment = Version::new(1, 0, 0);
        version += increment;
        assert_eq!(version.to_string(), "2.0.0");

        // マイナーバージョンの増加
        let increment = Version::new(0, 2, 0);
        version += increment;
        assert_eq!(version.to_string(), "2.2.0");

        // パッチバージョンの増加
        let increment = Version::new(0, 0, 3);
        version += increment;
        assert_eq!(version.to_string(), "2.2.3");
    }

    #[test]
    fn test_increment_mask() {
        let mut version = Version::new(1, 2, 3);

        // メジャーバージョンを1増加
        version.increment_mask("1.0.0").unwrap();
        assert_eq!(version.to_string(), "2.0.0");

        // マイナーバージョンを2増加
        version.increment_mask("0.2.0").unwrap();
        assert_eq!(version.to_string(), "2.2.0");

        // パッチバージョンを3増加
        version.increment_mask("0.0.3").unwrap();
        assert_eq!(version.to_string(), "2.2.3");

        // マスクのフォーマットエラー
        let result = version.increment_mask("x.y");
        assert!(result.is_err());
    }

    #[test]
    fn test_part_version() {
        // 正常なバージョン文字列
        let parts = part_version("1.2.3").unwrap();
        assert_eq!(parts, vec![1, 2, 3]);

        // 数値ではない部分を含むバージョン文字列
        let parts = part_version("1.a.3").unwrap();
        assert_eq!(parts, vec![1, 0, 3]);

        // 不正なフォーマット
        let result = part_version("1.2");
        assert!(result.is_err());
    }
}
