use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use thiserror::Error;

use semver::Version;

/// Represents possible errors that can occur during version processing
#[derive(Error, Debug)]
pub enum VersionError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Failed to open file '{path}': {source}")]
    FileOpen {
        path: String,
        source: io::Error,
    },
}

/// Represents a parsed line that may contain a semantic version
#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
pub enum VersionEntry {
    /// A valid semantic version
    SemVersion(Version),
    /// A line that doesn't contain a valid semantic version
    Unknown(String),
}

impl fmt::Display for VersionEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VersionEntry::SemVersion(v) => write!(f, "{v}"),
            VersionEntry::Unknown(s) => write!(f, "{s}"),
        }
    }
}

/// Reads and parses versions from a BufRead source
/// 
/// # Arguments
/// 
/// * `reader` - Any type that implements BufRead
/// 
/// # Returns
/// 
/// Returns a Result containing a vector of VersionEntry items or a VersionError
/// 
/// # Example
/// 
/// ```
/// use std::io::Cursor;
/// # use version::from_reader;
/// 
/// let input = "1.0.0\ninvalid\n2.0.0\n";
/// let mut cursor = Cursor::new(input);
/// let versions = from_reader(&mut cursor).unwrap();
/// assert_eq!(versions.len(), 3);
/// ```
fn from_reader<R: BufRead>(reader: &mut R) -> Result<Vec<VersionEntry>, VersionError> {
    let mut versions = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let entry = match Version::parse(&line) {
            Ok(v) => VersionEntry::SemVersion(v),
            Err(_) => VersionEntry::Unknown(line),
        };
        versions.push(entry);
    }

    Ok(versions)
}

/// Reads and parses versions from standard input
/// 
/// # Returns
/// 
/// Returns a Result containing a vector of VersionEntry items or a VersionError
/// 
/// # Example
/// 
/// ```no_run
/// # use version::from_stdin;
/// let versions = from_stdin().unwrap();
/// for version in versions {
///     println!("{}", version);
/// }
/// ```
pub fn from_stdin() -> Result<Vec<VersionEntry>, VersionError> {
    from_reader(&mut BufReader::new(io::stdin()))
}

/// Reads and parses versions from multiple files
/// 
/// # Arguments
/// 
/// * `paths` - A slice of file paths to process
/// 
/// # Returns
/// 
/// Returns a Result containing a vector of VersionEntry items or a VersionError
/// 
/// # Example
/// 
/// ```no_run
/// # use version::from_files;
/// let paths = vec!["versions.txt", "more_versions.txt"];
/// let versions = from_files(&paths).unwrap();
/// for version in versions {
///     println!("{}", version);
/// }
/// ```
pub fn from_files<P: AsRef<Path>>(paths: &[P]) -> Result<Vec<VersionEntry>, VersionError> {
    paths
        .iter()
        .map(|path| {
            File::open(path).map_err(|e| VersionError::FileOpen {
                path: path.as_ref().display().to_string(),
                source: e,
            })
        })
        .map(|file_result| {
            file_result.map(|file| from_reader(&mut BufReader::new(file)))
        })
        .try_fold(Vec::new(), |mut acc, file_result| {
            acc.extend(file_result??);
            Ok(acc)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_valid_version() {
        let input = "1.0.0\n";
        let mut cursor = Cursor::new(input);
        let versions = from_reader(&mut cursor).unwrap();
        assert_eq!(versions.len(), 1);
        assert!(matches!(versions[0], VersionEntry::SemVersion(_)));
    }

    #[test]
    fn test_parse_invalid_version() {
        let input = "not_a_version\n";
        let mut cursor = Cursor::new(input);
        let versions = from_reader(&mut cursor).unwrap();
        assert_eq!(versions.len(), 1);
        assert!(matches!(versions[0], VersionEntry::Unknown(_)));
    }

    #[test]
    fn test_parse_mixed_content() {
        let input = "1.0.0\ninvalid\n2.0.0\n";
        let mut cursor = Cursor::new(input);
        let versions = from_reader(&mut cursor).unwrap();
        assert_eq!(versions.len(), 3);
        assert!(matches!(versions[0], VersionEntry::SemVersion(_)));
        assert!(matches!(versions[1], VersionEntry::Unknown(_)));
        assert!(matches!(versions[2], VersionEntry::SemVersion(_)));
    }
}
