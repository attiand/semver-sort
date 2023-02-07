use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use semver::Version;

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone)]
pub enum Type {
    SemVersion(Version),
    Unknown(String),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::SemVersion(v) => write!(f, "{v}"),
            Type::Unknown(s) => write!(f, "{s}"),
        }
    }
}

fn from_reader<R: BufRead>(reader: &mut R) -> Result<Vec<Type>, String> {
    let mut versions: Vec<Type> = Vec::new();

    for line in reader.lines().flatten() {
        match Version::parse(&line) {
            Ok(v) => versions.push(Type::SemVersion(v)),
            Err(_) => versions.push(Type::Unknown(line)),
        }
    }

    Ok(versions)
}

pub fn from_stdin() -> Result<Vec<Type>, String> {
    from_reader(&mut BufReader::new(io::stdin()))
}

pub fn from_files(paths: Vec<&str>) -> Result<Vec<Type>, String> {
    let mut files: Vec<File> = Vec::new();

    for path in paths.iter() {
        match File::open(path) {
            Ok(f) => files.push(f),
            Err(e) => return Err(format!("Can't open: '{path}', {e}")),
        }
    }

    let mut versions: Vec<Type> = Vec::new();

    for file in files.iter() {
        match from_reader(&mut BufReader::new(file)) {
            Ok(v) => versions.extend(v.iter().cloned()),
            Err(e) => return Err(e),
        }
    }

    Ok(versions)
}
