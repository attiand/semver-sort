#[cfg(test)]
mod tests {

    use std::io::Error;
    use std::io::Write;
    use tempfile::NamedTempFile;

    enum Type {
        SEMVER,
        UNKNOWN,
    }

    fn assert(actual: &version::Type, expected_type: Type, expected_value: &str) {
        let m = match expected_type {
            Type::SEMVER => {
                matches!(actual, version::Type::SemVersion(v) if v.to_string() == expected_value)
            }
            Type::UNKNOWN => {
                matches!(actual, version::Type::Unknown(v) if v.to_string() == expected_value)
            }
        };

        assert!(m);
    }

    #[test]
    fn test_from_one_file() -> Result<(), Error> {
        let mut file = NamedTempFile::new()?;
        write!(file, "1.10.2\n1.9.3\nUNKNOWN")?;
        let versions = version::from_files(vec![file.path().to_str().unwrap()]).unwrap();

        assert_eq!(versions.len(), 3);
        assert(versions.get(0).unwrap(), Type::SEMVER, "1.10.2");
        assert(versions.get(1).unwrap(), Type::SEMVER, "1.9.3");
        assert(versions.get(2).unwrap(), Type::UNKNOWN, "UNKNOWN");

        Ok(())
    }

    #[test]
    fn test_from_two_files() -> Result<(), Error> {
        let mut file1 = NamedTempFile::new()?;
        write!(file1, "1.10.2\n1.9.3")?;

        let mut file2 = NamedTempFile::new()?;
        write!(file2, "2.10.2\n2.9.3")?;

        let versions = version::from_files(vec![
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
        ])
        .unwrap();

        assert_eq!(versions.len(), 4);
        assert(versions.get(0).unwrap(), Type::SEMVER, "1.10.2");
        assert(versions.get(1).unwrap(), Type::SEMVER, "1.9.3");
        assert(versions.get(2).unwrap(), Type::SEMVER, "2.10.2");
        assert(versions.get(3).unwrap(), Type::SEMVER, "2.9.3");
        Ok(())
    }

    #[test]
    fn test_sort() -> Result<(), Error> {
        let mut file = NamedTempFile::new()?;
        write!(file, "1.10.2\n1.9.3")?;
        let mut versions = version::from_files(vec![file.path().to_str().unwrap()]).unwrap();

        versions.sort();
        assert_eq!(versions.len(), 2);
        assert(versions.get(0).unwrap(), Type::SEMVER, "1.9.3");
        assert(versions.get(1).unwrap(), Type::SEMVER, "1.10.2");
        Ok(())
    }
}
