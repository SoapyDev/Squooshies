use std::path::PathBuf;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Paths {
    pub source: PathBuf,
    pub destination: PathBuf
}

impl Paths{
    pub fn is_valid(&self) -> bool{
        self.source.is_dir() && self.destination.is_dir() 
    }
}