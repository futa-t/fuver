use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::Path;

mod version;
use version::*;
mod build;
pub use build::*;

#[derive(Serialize, Deserialize)]
pub struct FuVer {
    #[serde(default)]
    pub version: Version,

    #[serde(default)]
    pub pre: Option<String>,

    #[serde()]
    pub build: Option<BuildIdentifiers>,
}

impl fmt::Display for FuVer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.version)?;
        if let Some(p) = &self.pre {
            write!(f, "-{}", p)?;
        }
        if let Some(b) = &self.build {
            write!(f, "{}", b)?;
        };
        write!(f, "")
    }
}

impl FuVer {
    fn part_mask(&self, mask: &str) -> Result<Vec<usize>, String> {
        let part = mask.split(".").map(|n| n.parse::<usize>().unwrap_or(0));
        Ok(part.collect())
    }

    pub fn increment_version(&mut self, mask: &str) -> Result<(), String> {
        let m_part: Vec<usize> = self.part_mask(mask)?;
        let m_version = Version::new(m_part[0], m_part[1], m_part[2]);

        self.version += m_version;

        Ok(())
    }

    pub fn increment_build(&mut self) -> Result<(), String> {
        if let Some(build) = &self.build {
            self.build = Some(build.increment()?);
        }
        Ok(())
    }

    pub fn show_version(&self) {
        println!("{}", self.version);
    }

    pub fn show_build(&self) {
        if let Some(b) = &self.build {
            println!("{}", b);
        } else {
            println!("no buildmetadata.");
        }
    }

    pub fn save(&self, p: &Path) {
        fs::write(p, toml::to_string(&self).unwrap()).unwrap();
    }
}
