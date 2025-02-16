use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct FuVer {
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub build: usize,
}

fn default_version() -> String {
    String::from("0.1.0")
}

impl fmt::Display for FuVer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}+build{}", self.version, self.build)
    }
}

impl FuVer {
    fn part_version(&self, version: &str) -> Result<Vec<usize>, String> {
        version
            .split(".")
            .map(|n| {
                n.parse::<usize>()
                    .map_err(|e| format!("バージョン文字列が不正です({}): {}", version, e))
            })
            .collect()
    }

    fn part_mask(&self, mask: &str) -> Result<Vec<usize>, String> {
        let part = mask.split(".").map(|n| n.parse::<usize>().unwrap_or(0));
        Ok(part.collect())
    }

    pub fn increment_version(&mut self, mask: &str) -> Result<(), String> {
        let v_part = self.part_version(&self.version)?;

        let m_part: Vec<usize> = self.part_mask(mask)?;

        let mut new_version: Vec<String> = vec![];
        for (v, m) in v_part.iter().zip(m_part.iter()) {
            new_version.push((v + m).to_string());
        }

        let new_version_string = new_version.join(".");
        self.version = new_version_string.clone();

        Ok(())
    }

    pub fn increment_build(&mut self) -> Result<(), String> {
        self.build += 1;
        Ok(())
    }

    pub fn save(&self, p: &Path) {
        fs::write(p, toml::to_string(&self).unwrap()).unwrap();
    }
}
