use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::io;
use std::result;
use std::str::FromStr;

use crate::buildmeta;
use crate::buildmeta::BuildMetaError;
use crate::pre;
use crate::pre::PreReleaseError;
use crate::version;

pub type Result<T> = result::Result<T, FuVerError>;
#[derive(Debug)]
pub enum FuVerError {
    IO(io::Error),
    InitError(String),
    Deserialize(toml::de::Error),
    Version(version::VersionError),
    Error(String),
}

impl fmt::Display for FuVerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FuVerError::IO(e) => write!(f, "I/O Error: {}", e),
            FuVerError::Deserialize(e) => write!(f, "Desirialize Error: {}", e),
            FuVerError::Version(e) => write!(f, "Version Error: {}", e),
            FuVerError::Error(e) => write!(f, "Error: {}", e),
            FuVerError::InitError(e) => write!(f, "Initialize Error: {}", e),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct FuVer {
    #[serde(default)]
    pub version: version::Version,

    #[serde(default)]
    pub pre: Option<pre::PreRelease>,

    #[serde(default)]
    pub build: Option<buildmeta::BuildMetaData>,
}

impl fmt::Display for FuVer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.version)?;
        if let Some(p) = self.pre.as_ref() {
            write!(f, "-{}", p)?;
        }
        if let Some(b) = self.build.as_ref() {
            write!(f, "+{}", b)?;
        }
        Ok(())
    }
}

impl FromStr for FuVer {
    type Err = FuVerError;
    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let fv = toml::from_str(s).map_err(FuVerError::Deserialize)?;
        Ok(fv)
    }
}

impl FuVer {
    pub fn save(&self, p: &str) {
        fs::write(p, toml::to_string(&self).unwrap()).unwrap();
    }

    pub fn incr_ver_major(&mut self) -> Result<()> {
        self.version.increment_major().map_err(FuVerError::Version)
    }

    pub fn incr_ver_minor(&mut self) -> Result<()> {
        self.version.increment_minor().map_err(FuVerError::Version)
    }

    pub fn incr_ver_patch(&mut self) -> Result<()> {
        self.version.increment_patch().map_err(FuVerError::Version)
    }

    pub fn incr_ver_mask(&mut self, mask: &str) -> Result<()> {
        self.version
            .increment_mask(mask)
            .map_err(FuVerError::Version)
    }

    pub fn incr_pre(&mut self) -> Result<()> {
        match self.pre.as_mut() {
            Some(p) => p.increment_number(),
            None => Err(PreReleaseError::Undefined),
        }
        .map_err(|e| FuVerError::Error(e.to_string()))
    }

    pub fn incr_build_num(&mut self) -> Result<()> {
        match self.build.as_mut() {
            Some(b) => b.increment_number(),
            None => Err(BuildMetaError::Undefined),
        }
        .map_err(|e| FuVerError::Error(e.to_string()))
    }

    pub fn incr_build_date(&mut self) -> Result<()> {
        match self.build.as_mut() {
            Some(b) => b.update_date(),
            None => Err(BuildMetaError::Undefined),
        }
        .map_err(|e| FuVerError::Error(e.to_string()))
    }

    pub fn incr_build_hash(&mut self) -> Result<()> {
        match self.build.as_mut() {
            Some(b) => b.update_hash(),
            None => Err(BuildMetaError::Undefined),
        }
        .map_err(|e| FuVerError::Error(e.to_string()))
    }

    pub fn incr_build_all(&mut self) -> Result<()> {
        self.incr_build_num()?;
        self.incr_build_date()?;
        self.incr_build_hash()?;
        Ok(())
    }
}
