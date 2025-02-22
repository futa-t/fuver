use std::fmt;

use chrono::{DateTime, Local};
use git2::Repository;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct BuildMetaData {
    pub number: usize,
    pub date: String,
    pub hash: String,
    format: String,
}

#[derive(Debug)]
pub struct BuildMetaError {
    msg: String,
}

impl From<String> for BuildMetaError {
    fn from(message: String) -> Self {
        BuildMetaError { msg: message }
    }
}

impl fmt::Display for BuildMetaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
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
    pub fn show_all(&self) {
        println!("{}", &self);
    }

    /// Print from Config Format
    pub fn show(&self) {
        println!("{}", self.create_string());
    }

    fn create_string(&self) -> String {
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
    fn fmt_replace(&self, opt: &str) -> Result<String, BuildMetaError> {
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
                return Err(BuildMetaError::from(opt.to_string()));
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
    pub fn show_fmt(&self, fmt: &str) {
        let t = self.create_fmt_string(fmt);
        println!("{}", t);
    }

    fn create_fmt_string(&self, fmt: &str) -> String {
        let mut result = String::from("+");
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
                while let Some(o) = chars.next() {
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
        result.to_string()
    }

    /// Increment BuildNumber
    ///
    /// # Errors
    /// Overflow BuildNumber.
    pub fn increment_number(&mut self) -> Result<(), BuildMetaError> {
        self.number = self
            .number
            .checked_add(1)
            .ok_or(BuildMetaError::from("Number overflow".to_string()))?;
        Ok(())
    }

    /// Update BuildDate to today
    pub fn update_date(&mut self) -> Result<(), BuildMetaError> {
        self.date = Local::now().to_rfc3339();
        Ok(())
    }

    /// Update BuildHash.
    ///
    /// # Errors
    /// git2::Error
    pub fn update_hash(&mut self) -> Result<(), BuildMetaError> {
        let hash = get_commit_hash().map_err(|e| BuildMetaError::from(e.to_string()))?;
        self.hash = hash;
        Ok(())
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
            hash: get_commit_hash().unwrap_or(String::new()),
            format: "{number}.{date:%Y%m%d}.{hash:8}".to_string(),
        }
    }
}

impl fmt::Display for BuildMetaData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.create_string())
    }
}

fn get_commit_hash() -> Result<String, git2::Error> {
    let repo = Repository::open(".")?;
    let head = repo.head()?;
    let commit = head.peel_to_commit()?;
    Ok(commit.id().to_string())
}
