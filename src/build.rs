use core::fmt;
use std::fmt::Debug;

use chrono::{Local, NaiveDate};
use git2::Repository;
use serde::{de::Visitor, Deserialize, Serialize};

pub enum BuildIdentifiers {
    BuildNumber(usize),
    CommitHash(String),
    DateTime(String),
}

impl Serialize for BuildIdentifiers {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            BuildIdentifiers::BuildNumber(n) => serializer.serialize_i32(*n as i32),
            BuildIdentifiers::DateTime(dt) => serializer.serialize_str(dt),
            BuildIdentifiers::CommitHash(h) => serializer.serialize_str(h),
        }
    }
}

impl<'de> Deserialize<'de> for BuildIdentifiers {
    fn deserialize<D>(deserializer: D) -> Result<BuildIdentifiers, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(IdentifierVisitor)
    }
}

pub fn is_date(s: &str) -> bool {
    NaiveDate::parse_from_str(s, "%Y%m%d").is_ok()
}

struct IdentifierVisitor;
impl<'de> Visitor<'de> for IdentifierVisitor {
    type Value = BuildIdentifiers;

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if is_date(&v.to_string()) {
            Ok(BuildIdentifiers::DateTime(v.to_string()))
        } else {
            Ok(BuildIdentifiers::BuildNumber(v as usize))
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if is_date(v) {
            Ok(BuildIdentifiers::DateTime(v.to_string()))
        } else {
            Ok(BuildIdentifiers::CommitHash(v.to_string()))
        }
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("")
    }
}

impl Debug for BuildIdentifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BuildNumber(arg0) => f.debug_tuple("BuildNumber").field(arg0).finish(),
            Self::CommitHash(arg0) => f.debug_tuple("CommitHash").field(arg0).finish(),
            Self::DateTime(arg0) => f.debug_tuple("DateTime").field(arg0).finish(),
        }
    }
}

impl fmt::Display for BuildIdentifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildIdentifiers::BuildNumber(n) => write!(f, "+build.{}", n),
            BuildIdentifiers::DateTime(dt) => write!(f, "+date.{}", dt),
            BuildIdentifiers::CommitHash(h) => write!(f, "+hash.{}", h),
        }
    }
}

impl BuildIdentifiers {
    pub fn increment(&self) -> Result<Self, String> {
        let new_build = match self {
            BuildIdentifiers::BuildNumber(n) => BuildIdentifiers::BuildNumber(*n + 1),
            BuildIdentifiers::CommitHash(_) => self.new_commit_hash().map_err(|e| e.to_string())?,
            BuildIdentifiers::DateTime(_) => self.new_date()?,
        };

        Ok(new_build)
    }

    fn new_date(&self) -> Result<Self, String> {
        let new_date = Local::now().format("%Y%m%d").to_string();
        Ok(BuildIdentifiers::DateTime(new_date))
    }
    fn new_commit_hash(&self) -> Result<Self, git2::Error> {
        let repo = Repository::open(".")?;
        let head = repo.head()?;
        let commit = head.peel_to_commit()?;
        let hash = commit.id().to_string();
        Ok(BuildIdentifiers::CommitHash(hash))
    }
}
