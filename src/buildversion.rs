use crate::version::Version;

use std::fmt;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Read, Write};
use std::path::PathBuf;
pub struct BuildVerion {
    path: PathBuf,
}

impl Version for BuildVerion {
    fn new(outdir: &str) -> Self {
        let outdir = PathBuf::from(outdir);

        let file = if !outdir.exists() {
            if let Err(_) = fs::create_dir_all(&outdir) {
                PathBuf::from(".")
            } else {
                outdir
            }
        } else {
            outdir
        };

        BuildVerion {
            path: file.join("buildversion"),
        }
    }

    fn increment(&self) -> std::io::Result<String> {
        let version = self.version()?.parse::<usize>().unwrap_or(0) + 1;

        self.write_file(&version.to_string())?;

        Ok(version.to_string())
    }

    fn version(&self) -> io::Result<String> {
        let mut version = String::new();
        if let Ok(mut f) = File::open(&self.path) {
            f.read_to_string(&mut version)?;
        } else {
            version = "0".to_string();
            self.write_file(&version)?;
        }
        Ok(version)
    }

    fn write_file(&self, version: &str) -> io::Result<()> {
        let f = File::create(&self.path)?;
        let mut bf = BufWriter::new(f);
        bf.write(version.to_string().as_bytes()).map_err(|e| {
            std::io::Error::new(
                e.kind(),
                format!(
                    "ビルド番号の書き込みに失敗しました。現在のビルド番号は {} です。",
                    version
                ),
            )
        })?;
        Ok(())
    }
}

impl fmt::Display for BuildVerion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.version() {
            Ok(v) => {
                write!(f, "+build{}", v)
            }
            Err(e) => {
                write!(f, "{}", e.to_string())
            }
        }
    }
}
