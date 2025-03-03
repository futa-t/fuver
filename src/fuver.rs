use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
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
    PreReleaseNotDefined,
    BuildMetaDataNotDefined,
}

impl fmt::Display for FuVerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FuVerError::IO(e) => write!(f, "I/O Error: {}", e),
            FuVerError::Deserialize(e) => write!(f, "Desirialize Error: {}", e),
            FuVerError::Version(e) => write!(f, "Version Error: {}", e),
            FuVerError::Error(e) => write!(f, "Error: {}", e),
            FuVerError::InitError(e) => write!(f, "Initialize Error: {}", e),
            FuVerError::PreReleaseNotDefined => write!(f, "Pre-Release is Not Defined."),
            FuVerError::BuildMetaDataNotDefined => write!(f, "BuildMetaData is Not Defined."),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct FuVer {
    #[serde(default)]
    version: version::Version,

    #[serde(default)]
    pre: Option<pre::PreRelease>,

    #[serde(default)]
    build: Option<buildmeta::BuildMetaData>,
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

    pub fn incr_ver_major(&mut self, silent: bool) -> Result<()> {
        Self::set_helper(
            &mut self.version,
            |v| v.increment_major().map_err(FuVerError::Version),
            "Increment Major Version",
            silent,
        )
    }
    pub fn incr_ver_minor(&mut self, silent: bool) -> Result<()> {
        Self::set_helper(
            &mut self.version,
            |v| v.increment_minor().map_err(FuVerError::Version),
            "Increment Minor Version",
            silent,
        )
    }

    pub fn incr_ver_patch(&mut self, silent: bool) -> Result<()> {
        Self::set_helper(
            &mut self.version,
            |v| v.increment_patch().map_err(FuVerError::Version),
            "Increment Patch Version",
            silent,
        )
    }

    pub fn incr_ver_mask(&mut self, mask: &str, silent: bool) -> Result<()> {
        Self::set_helper(
            &mut self.version,
            |v| v.increment_mask(mask).map_err(FuVerError::Version),
            "Increment Version",
            silent,
        )
    }

    pub fn incr_pre(&mut self, silent: bool) -> Result<()> {
        let pre = self.pre.get_or_insert_with(pre::PreRelease::default);
        Self::set_helper(
            pre,
            |p| {
                p.increment_number()
                    .map_err(|e| FuVerError::Error(e.to_string()))
            },
            "Increment Pre-Release",
            silent,
        )
    }

    pub fn incr_build_num(&mut self, silent: bool) -> Result<()> {
        let build = self
            .build
            .get_or_insert_with(buildmeta::BuildMetaData::default);
        Self::set_helper(
            build,
            |b| {
                b.increment_number()
                    .map_err(|e| FuVerError::Error(e.to_string()))
            },
            "Increment Build Number",
            silent,
        )
    }
    pub fn incr_build_date(&mut self, silent: bool) -> Result<()> {
        let build = self
            .build
            .get_or_insert_with(buildmeta::BuildMetaData::default);
        Self::set_helper(
            build,
            |b| {
                b.update_date()
                    .map_err(|e| FuVerError::Error(e.to_string()))
            },
            "Increment Build Date",
            silent,
        )
    }
    pub fn incr_build_hash(&mut self, silent: bool) -> Result<()> {
        let build = self
            .build
            .get_or_insert_with(buildmeta::BuildMetaData::default);
        Self::set_helper(
            build,
            |b| {
                b.update_hash()
                    .map_err(|e| FuVerError::Error(e.to_string()))
            },
            "Increment Build Hash",
            silent,
        )
    }

    pub fn incr_build_all(&mut self, silent: bool) -> Result<()> {
        let current = {
            let b = self.get_build()?;
            b.to_string()
        };
        self.incr_build_num(false)?;
        self.incr_build_date(false)?;
        self.incr_build_hash(false)?;
        if !silent {
            let new = self.get_build()?;
            println!("Increment Build {} -> {}", current, new);
        }
        Ok(())
    }

    pub fn show_version(&self) -> Result<()> {
        println!("{}", &self.version);
        Ok(())
    }

    pub fn show_major(&self) -> Result<()> {
        self.version
            .show_major()
            .map_err(|e| FuVerError::Error(e.to_string()))
    }

    pub fn show_minor(&self) -> Result<()> {
        self.version
            .show_minor()
            .map_err(|e| FuVerError::Error(e.to_string()))
    }

    pub fn show_patch(&self) -> Result<()> {
        self.version
            .show_patch()
            .map_err(|e| FuVerError::Error(e.to_string()))
    }

    fn get_prerelease(&self) -> Result<&pre::PreRelease> {
        self.pre.as_ref().ok_or(FuVerError::PreReleaseNotDefined)
    }

    pub fn show_prerelease(&self) -> Result<()> {
        self.get_prerelease()?
            .show()
            .map_err(|e| FuVerError::Error(e.to_string()))
    }

    pub fn show_prerelease_tag(&self) -> Result<()> {
        self.get_prerelease()?
            .show_tag()
            .map_err(|e| FuVerError::Error(e.to_string()))
    }

    pub fn show_prerelease_number(&self) -> Result<()> {
        self.get_prerelease()?
            .show_number()
            .map_err(|e| FuVerError::Error(e.to_string()))
    }

    fn get_build(&self) -> Result<&buildmeta::BuildMetaData> {
        self.build
            .as_ref()
            .ok_or(FuVerError::BuildMetaDataNotDefined)
    }

    pub fn show_build_fmt(&self, fmt: &str) -> Result<()> {
        self.get_build()?
            .show_fmt(fmt)
            .map_err(|_| FuVerError::BuildMetaDataNotDefined)
    }

    pub fn show_build(&self) -> Result<()> {
        self.get_build()?
            .show()
            .map_err(|e| FuVerError::Error(e.to_string()))
    }

    pub fn show_build_number(&self) -> Result<()> {
        self.get_build()?
            .show_number()
            .map_err(|e| FuVerError::Error(e.to_string()))
    }

    pub fn show_build_date(&self) -> Result<()> {
        self.get_build()?
            .show_date()
            .map_err(|e| FuVerError::Error(e.to_string()))
    }

    pub fn show_build_hash(&self) -> Result<()> {
        self.get_build()?
            .show_hash()
            .map_err(|e| FuVerError::Error(e.to_string()))
    }

    pub fn show_build_all(&self) -> Result<()> {
        self.get_build()?
            .show_all()
            .map_err(|e| FuVerError::Error(e.to_string()))
    }

    pub fn show_full(&self) -> Result<()> {
        println!("{}", &self);
        Ok(())
    }

    fn set_helper<T, F>(target: &mut T, action: F, head: &str, silent: bool) -> Result<()>
    where
        T: Display,
        F: FnOnce(&mut T) -> Result<()>,
    {
        let current_value = target.to_string();
        action(target)?;
        if !silent {
            println!("{} {} -> {}", head, current_value, target);
        };
        Ok(())
    }

    pub fn set_version(&mut self, s: &str, silent: bool) -> Result<()> {
        Self::set_helper(
            &mut self.version,
            |v| v.set(s).map_err(|e| FuVerError::Error(e.to_string())),
            "Set Vesion",
            silent,
        )
    }

    pub fn set_major(&mut self, n: usize, silent: bool) -> Result<()> {
        Self::set_helper(
            &mut self.version,
            |v| v.set_major(n).map_err(|e| FuVerError::Error(e.to_string())),
            "Set Major Version",
            silent,
        )
    }

    pub fn set_minor(&mut self, n: usize, silent: bool) -> Result<()> {
        Self::set_helper(
            &mut self.version,
            |v| v.set_minor(n).map_err(|e| FuVerError::Error(e.to_string())),
            "Set Minor Version",
            silent,
        )
    }

    pub fn set_patch(&mut self, n: usize, silent: bool) -> Result<()> {
        Self::set_helper(
            &mut self.version,
            |v| v.set_patch(n).map_err(|e| FuVerError::Error(e.to_string())),
            "Set Patch Version",
            silent,
        )
    }

    pub fn set_pre(&mut self, tag: &str, number: Option<usize>, silent: bool) -> Result<()> {
        let pre = self.pre.get_or_insert_with(pre::PreRelease::default);
        Self::set_helper(
            pre,
            |p| {
                p.set(tag, number)
                    .map_err(|e| FuVerError::Error(e.to_string()))
            },
            "Set Pre-Release",
            silent,
        )
    }

    pub fn set_build_number(&mut self, n: usize, silent: bool) -> Result<()> {
        let build = self
            .build
            .get_or_insert_with(buildmeta::BuildMetaData::default);
        Self::set_helper(
            build,
            |b| {
                b.set_number(n)
                    .map_err(|e| FuVerError::Error(e.to_string()))
            },
            "Set Build Number",
            silent,
        )
    }
    pub fn set_build_date(&mut self, date: &str, silent: bool) -> Result<()> {
        let build = self
            .build
            .get_or_insert_with(buildmeta::BuildMetaData::default);
        Self::set_helper(
            build,
            |b| {
                b.set_date(date)
                    .map_err(|e| FuVerError::Error(e.to_string()))
            },
            "Set Build Date",
            silent,
        )
    }

    pub fn set_build_hash(&mut self, hash: &str, silent: bool) -> Result<()> {
        let build = self
            .build
            .get_or_insert_with(buildmeta::BuildMetaData::default);
        Self::set_helper(
            build,
            |b| {
                b.set_hash(hash)
                    .map_err(|e| FuVerError::Error(e.to_string()))
            },
            "Set Build Hash",
            silent,
        )
    }

    pub fn set_build_fmt(&mut self, fmt: &str, silent: bool) -> Result<()> {
        let build = self
            .build
            .get_or_insert_with(buildmeta::BuildMetaData::default);
        let current = build.get_format();
        build
            .set_format(fmt)
            .map_err(|e| FuVerError::Error(e.to_string()))?;
        if !silent {
            println!("Set Build Format {} -> {}", current, build.get_format());
        }
        Ok(())
    }
}
