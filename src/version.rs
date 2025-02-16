use std::io;

pub trait Version {
    fn new(outdir: &str) -> Self;
    fn increment(&self) -> io::Result<String>;
    fn version(&self) -> io::Result<String>;
    fn write_file(&self, version: &str) -> io::Result<()>;
}
