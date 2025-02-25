use std::{fmt, result};

use crate::identifier;
use chrono::{DateTime, Local};
use git2::Repository;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct BuildMetaData {
    pub number: usize,
    date: String,
    hash: String,
    format: String,
}

pub type Result<T> = result::Result<T, BuildMetaError>;

#[derive(Debug)]
pub enum BuildMetaError {
    Format(String),
    Overflow(usize),
    Git(String),
    Date,
}

impl fmt::Display for BuildMetaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildMetaError::Format(s) => write!(f, "フォーマットが不正です: {}", s),
            BuildMetaError::Git(s) => write!(f, "Git情報の取得に失敗しました: {}", s),
            BuildMetaError::Date => write!(f, "日時の取得に失敗しました"),
            BuildMetaError::Overflow(n) => write!(f, "数値が指定できる範囲を超えています: {}+1", n),
        }
    }
}

impl BuildMetaData {
    pub fn new(number: usize, date: String, hash: String, format: String) -> BuildMetaData {
        BuildMetaData {
            number,
            date,
            hash,
            format,
        }
    }

    /// Print Full BuildMetaData
    /// example: `+build.123.20250220.fef16c61`
    pub fn show_all(&self) -> Result<()> {
        println!("{}", &self);
        Ok(())
    }

    /// Print from Config Format
    pub fn show(&self) -> Result<()> {
        match self.create_string() {
            Ok(s) => {
                println!("{}", s);
                Ok(())
            }
            Err(e) => Err(BuildMetaError::Format(e.to_string())),
        }
    }

    fn create_string(&self) -> identifier::Result<String> {
        self.create_fmt_string(&self.format)
    }

    // Note: なぜ`str.replace`やRegexクレートを使わずに置換処理を作成したのか
    //
    // `str.replace`による置換の場合、複数の書式において同じ値に置換する際に列挙することになり
    // `hash`の桁数指定では単純な置換はできず`str.replace`以外の処理を差し込む必要がある。
    // また、今後の改良として`date`で書式指定をできるようにすることも考えているため
    // 最終的に単純な置換がとおるのは`number`だけになる。
    // これらのことから`str.replace`による実装はスマートではない。
    //
    // そこで正規表現のキャプチャを使ったパースを利用することを考える。
    // これは一見良い選択肢に思えるがそれは標準ライブラリとして正規表現を扱える場合においての話である。
    // 現状で正規表現の必要性があるのはこの部分だけであり現段階での見通しでも利用箇所が増えることは想像しにくい。
    // よって依存を増やすよりも自前の置換処理で対応するほうが良い。
    //
    // この規模のプロジェクトにおいてはクレートを導入することによって自前のコードはたしかに削減されるが
    // それによって依存によるプロジェクトの総コード量は増えてしまうことに留意する必要がある。
    //
    // すでに実際の処理が思い浮かんでいるのならば、あきらかにニーズに大して膨大であろうドキュメントや
    // ソースからそれらの使い方やハンドリングなどを調べるより自分でで実装するほうが大体の場合手早くすむ場合が多い。
    //
    // ただし、ウンウンと頭を捻りながら数時間悩むのならば話は別である。こねくりまわして作成したコードは大体のちに悩みの種になるので。
    //
    // 現在依存している`clap`や`serde`、`toml`に関しても最終的には依存を外して自前実装にするつもり。
    fn fmt_replace(&self, opt: &str) -> Result<String> {
        let o: Vec<&str> = opt.split(":").collect();

        let ret = match o[0] {
            "number" | "num" | "n" => self.number.to_string(),
            "date" | "d" => match DateTime::parse_from_rfc3339(&self.date) {
                Ok(dt) => {
                    let f = match o.get(1..) {
                        Some(s) => &s.join(":"),
                        None => "%Y%m%d",
                    };
                    dt.format(f).to_string()
                }
                Err(_) => self.date.clone().to_string(),
            },
            "hash" | "h" => {
                let n = match o.get(1) {
                    Some(s) => s.parse::<usize>().unwrap_or(8),
                    None => 8,
                };
                self.hash_haed(n)
            }
            _ => {
                return Err(BuildMetaError::Format(opt.to_string()));
            }
        };
        Ok(ret)
    }

    /// Print BuildMetaData with format-string
    ///
    /// ## formats
    /// | format               | input                            | export                         | note                                                  |
    /// | -------------------- | -------------------------------- | ------------------------------ | ----------------------------------------------------- |
    /// | number,<br>num,<br>n | `build.{number}`<br>`build{num}` | `build.123`<br>`build123`      |                                                       |
    /// | date,<br>d           | `date.{date}`<br>`{d:%Y/%m/%d %H:%M}`           | `date.20250220`<br>`20250220`  | future: strftime support                              |
    /// | hash,<br>h           | `hash.{hash}`<br>`{hash:4}`      | `hash.fef16c61`<br>`hash.fef1` | After `:`, specify display digits (default: 8 digits) |
    pub fn show_fmt(&self, fmt: &str) -> Result<()> {
        let t = self
            .create_fmt_string(fmt)
            .map_err(|e| BuildMetaError::Format(e.to_string()))?;
        println!("{}", t);
        Ok(())
    }

    fn create_fmt_string(&self, fmt: &str) -> identifier::Result<String> {
        let mut result = String::new();
        let mut chars = fmt.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '\\' {
                if let Some(c) = chars.next() {
                    result.push(c);
                }
            } else if ch == '{' {
                let mut opt = String::new();
                let mut tmp = String::new();
                tmp.push(ch);
                for o in chars.by_ref() {
                    tmp.push(o);
                    if o == '}' {
                        break;
                    };
                    opt.push(o);
                }
                let push_str = match self.fmt_replace(&opt) {
                    Ok(s) => s,
                    Err(_) => tmp,
                };
                result.push_str(&push_str);
            } else {
                result.push(ch);
            }
        }
        identifier::check_dot_separated_identifiers(&result)?;
        Ok(result.to_string())
    }

    /// Increment BuildNumber
    ///
    /// # Errors
    /// Overflow BuildNumber.
    pub fn increment_number(&mut self) -> Result<()> {
        self.number = self
            .number
            .checked_add(1)
            .ok_or(BuildMetaError::Overflow(self.number))?;
        Ok(())
    }

    /// Update BuildDate to today
    pub fn update_date(&mut self) -> Result<()> {
        self.date = Local::now().to_rfc3339();
        Ok(())
    }

    /// Update BuildHash.
    ///
    /// # Errors
    /// git2::Error
    pub fn update_hash(&mut self) -> Result<()> {
        let hash = get_commit_hash().map_err(|e| BuildMetaError::Git(e.to_string()))?;
        self.hash = hash;
        Ok(())
    }

    pub fn get_number(&self) -> usize {
        self.number
    }

    pub fn get_date(&self) -> String {
        self.date.clone()
    }

    pub fn get_hash(&self) -> String {
        self.hash.to_string()
    }

    fn hash_haed(&self, size: usize) -> String {
        self.hash.get(..size).unwrap_or(&self.hash).to_string()
    }
}

impl Default for BuildMetaData {
    /// Returns the default value.
    ///
    /// |        |                     |
    /// | ------ | ------------------- |
    /// | number | 0                   |
    /// | date   | today(%Y%m%d)       |
    /// | hash   | current commit hash |
    fn default() -> Self {
        Self {
            number: Default::default(),
            date: Local::now().to_rfc3339(),
            hash: get_commit_hash().unwrap_or_default(),
            format: "{number}.{date:%Y%m%d}.{hash:8}".to_string(),
        }
    }
}

impl fmt::Display for BuildMetaData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.create_string() {
            Ok(s) => write!(f, "{}", s),
            Err(e) => write!(f, "{}", e),
        }
    }
}

fn get_commit_hash() -> result::Result<String, git2::Error> {
    let repo = Repository::open(".")?;
    let head = repo.head()?;
    let commit = head.peel_to_commit()?;
    Ok(commit.id().to_string())
}
