use std::fmt;

use chrono::Local;
use git2::Repository;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BuildMetaData {
    number: usize,
    date: String,
    hash: String,
}

impl BuildMetaData {
    pub fn show(&self) {
        println!("{}", &self);
    }

    /// フォーマットに対応したプロパティの文字列
    ///
    /// # Arguments
    ///
    /// * `opt` - フォーマット文字:
    ///   - 'n': number ビルド番号
    ///   - 'd': date   日時
    ///   - 'h': hash   コミットハッシュ
    ///   - その他: そのままの文字
    ///
    /// # Returns
    ///
    /// フォーマットに対応したプロパティの文字列
    fn opt_convert(&self, opt: char) -> String {
        match opt {
            'n' => self.number.to_string(),
            'd' => self.date.clone(),
            'h' => self.hash8(),
            _ => opt.to_string(),
        }
    }

    pub fn show_fmt(&self, fmt: &str) {
        let mut fmt_string = String::from("+");
        let mut optflg = false;
        for ch in fmt.chars() {
            if ch == '%' {
                optflg = true;
            } else if optflg {
                optflg = false;
                let s = self.opt_convert(ch);
                fmt_string.push_str(&s);
            } else {
                optflg = false;
                fmt_string.push(ch);
            }
        }
        println!("{}", &fmt_string);
    }

    pub fn increment_number(&mut self) -> Result<(), String> {
        self.number = self.number + 1;
        Ok(())
    }

    pub fn update_date(&mut self) -> Result<(), String> {
        self.date = Local::now().format("%Y%m%d").to_string();
        Ok(())
    }

    pub fn update_hash(&mut self) -> Result<(), String> {
        let hash = get_commit_hash().map_err(|e| e.to_string())?;
        self.hash = hash;
        Ok(())
    }

    fn hash8(&self) -> String {
        let h = &self.hash[..8];
        h.to_string()
    }
}

impl Default for BuildMetaData {
    fn default() -> Self {
        Self {
            number: Default::default(),
            date: Local::now().format("%Y%m%d").to_string(),
            hash: get_commit_hash().unwrap_or(String::new()),
        }
    }
}

impl fmt::Display for BuildMetaData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "+build.{}", &self.number)?;
        write!(f, ".{}", &self.date)?;
        write!(f, ".{}", &self.hash[..8])?;
        write!(f, "")
    }
}

fn get_commit_hash() -> Result<String, git2::Error> {
    let repo = Repository::open(".")?;
    let head = repo.head()?;
    let commit = head.peel_to_commit()?;
    Ok(commit.id().to_string())
}
