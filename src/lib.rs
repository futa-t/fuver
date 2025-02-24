use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::Path;

mod version;
use version::*;
mod buildmeta;
pub use buildmeta::*;
mod show;
pub use show::*;
mod increment;
pub use increment::*;
mod pre;

mod identifier;

#[derive(Serialize, Deserialize)]
pub struct FuVer {
    #[serde(default)]
    pub version: Version,

    #[serde(default)]
    pub pre: Option<pre::PreRelease>,

    #[serde(default)]
    pub build: BuildMetaData,
}

impl fmt::Display for FuVer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.version)?;
        if let Some(p) = &self.pre {
            write!(f, "{}", p)?;
        }
        write!(f, "{}", self.build)
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
        self.build.increment_number().map_err(|e| e.to_string())
    }

    pub fn show_version(&self) {
        println!("{}", self.version);
    }

    pub fn show_build(&self) {
        println!("{}", self.build);
    }

    pub fn save(&self, p: &Path) {
        fs::write(p, toml::to_string(&self).unwrap()).unwrap();
    }

    pub fn show_prerelease(&self) {
        if let Some(pre) = &self.pre {
            pre.show();
        }
    }
}
