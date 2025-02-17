use serde::{Deserialize, Serialize};
use std::{fmt, ops::AddAssign, str::FromStr};

fn part_version(version: &str) -> Result<Vec<usize>, String> {
    version
        .split(".")
        .map(|n| {
            n.parse::<usize>()
                .map_err(|e| format!("バージョン文字列が不正です({}): {}", version, e))
        })
        .collect()
}
pub struct Version {
    major: usize,
    minor: usize,
    patch: usize,
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Version::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl FromStr for Version {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = part_version(&s)?;
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
}
