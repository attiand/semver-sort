use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use semver::{Version, VersionReq};

use clap::{IntoApp, Parser};
use clap_complete::{generate, shells::Bash};
use clap_mangen;

const STDIN_FILE_NAME: &str = "-";

/// Print, filter, sort lines that match a semantic version (https://semver.org).
///
/// Print lines that match a semantic version to standard output. With no FILE, or when FILE is '-', read standard input.
#[derive(Parser)]
#[clap(author, version, about)]

struct Args {
    /// Sort lines
    #[clap(short, long, takes_value(false))]
    sort: bool,

    /// Sort lines in reversed order
    #[clap(short, long, takes_value(false))]
    reverse: bool,

    /// Removes repeated versions (implies --sort)
    #[clap(short, long, takes_value(false))]
    uniq: bool,

    /// Filter versions acording to expression. Has no meaning with --invert
    #[clap(short, long, value_name("EXPR"))]
    filter: Option<String>,

    /// Invert match, i.e. print lines that not match a semantic version
    #[clap(short, long, takes_value(false))]
    invert: bool,

    /// Generate bash completion and exit
    #[clap(long, takes_value(false))]
    completion: bool,

    /// Generate manual page
    #[clap(long, hide(true), takes_value(false))]
    manual: bool,

    /// Files to process, if '-' read standard input
    files: Vec<String>,
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum VersionType {
    SemVersion(Version),
    Unknown(String),
}

impl fmt::Display for VersionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            VersionType::SemVersion(v) => write!(f, "{}", v),
            VersionType::Unknown(s) => write!(f, "{}", s),
        }
    }
}

fn filter(req: &Option<VersionReq>, ver: &Version) -> bool {
    match req  {
        Some(r) => r.matches(&ver),
        None => true
    }
}

fn main() -> Result<(), String> {
    let mut args = Args::parse();
    let mut command = Args::command();

    if args.completion {
        generate(Bash, &mut command, "semver", &mut io::stdout());
        return Ok(());
    }

    if args.manual {
        return match clap_mangen::Man::new(command).render(&mut io::stdout()) {
            Ok(()) => Ok(()),
            Err(msg) => Err(msg.to_string()),
        };
    }

    let requirement = match args.filter {
        Some(f) => match VersionReq::parse(&f) {
            Ok(requirement) => Some(requirement),
            Err(e) => return Err(format!("{}{}", "Illegal format expression: ", e.to_string())),
        },
        None => None,
    };

    if args.files.is_empty() {
        args.files.push(STDIN_FILE_NAME.to_string())
    }

    let mut versions: Vec<VersionType> = Vec::new();

    for name in args.files.iter() {
        let reader: Box<dyn BufRead> = match name.as_ref() {
            STDIN_FILE_NAME => Box::new(BufReader::new(io::stdin())),
            _ => match File::open(name) {
                Ok(file) => Box::new(BufReader::new(file)),
                Err(e) => return Err(e.to_string()),
            },
        };

        for line in reader.lines() {
            if let Ok(l) = line {
                let version = Version::parse(&l);
                match version {
                    Ok(v) => versions.push(VersionType::SemVersion(v)),
                    Err(_) => versions.push(VersionType::Unknown(l)),
                }
            }
        }
    }

    if !args.invert {
        versions.retain(|c| matches!(c, VersionType::SemVersion(v) if filter(&requirement, v)))
    } else {
        versions.retain(|c| matches!(c, VersionType::Unknown(_)))
    }

    if args.sort || args.uniq || args.reverse {
        versions.sort()
    }

    if args.uniq {
        versions.dedup()
    }

    if args.reverse {
        versions.reverse()
    }

    for version in versions.iter() {
        println!("{}", version)
    }

    Ok(())
}
