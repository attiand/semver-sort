use std::fs::File;
use std::io::{self, BufRead, BufReader};

use semver::Version;

use clap::{IntoApp, Parser};
use clap_complete::{generate, shells::Bash};

const STDIN_FILE_NAME: &str = "-";

macro_rules! filename {
    ($name:expr) => {
        if $name == STDIN_FILE_NAME {
            "<stdin>"
        } else {
            $name
        }
    };
}

/// Sort lines of text files according to semantic versioning.
///
/// Write sorted lines to standard output. With no FILE, or when FILE is '-', read standard input.
#[derive(Parser)]
#[clap(author, version, about)]

struct Args {
    /// Reverse the result of comparisons
    #[clap(short, long, takes_value = false)]
    reverse: bool,

    /// Removes repeated versions
    #[clap(short, long, takes_value = false)]
    uniq: bool,

    /// Silently ignore lines with unrecognized versions
    #[clap(short, long, takes_value = false)]
    ignore: bool,

    /// Fail for any unrecognized versions
    #[clap(short, long, takes_value = false)]
    fail: bool,

    /// Generate bash completion and exit
    #[clap(long, takes_value = false)]
    completion: bool,

    /// Files to sort, if '-' read standard input
    files: Vec<String>,
}

fn main() -> Result<(), String> {
    let mut args = Args::parse();
    let mut command = Args::command();

    if args.completion {
        generate(Bash, &mut command, "semver-sort", &mut io::stdout());
        return Ok(());
    }

    if args.files.is_empty() {
        args.files.push(STDIN_FILE_NAME.to_string())
    }

    let mut versions: Vec<Version> = Vec::new();

    for name in args.files.iter() {
        let reader: Box<dyn BufRead> = match name.as_ref() {
            STDIN_FILE_NAME => Box::new(BufReader::new(io::stdin())),
            _ => match File::open(name) {
                Ok(file) => Box::new(BufReader::new(file)),
                Err(e) => return Err(e.to_string()),
            },
        };

        for (index, line) in reader.lines().enumerate() {
            if let Ok(l) = line {
                let version = Version::parse(&l);
                match version {
                    Ok(v) => {
                        versions.push(v);
                    }
                    Err(m) => {
                        let msg = format!("{}:{}: {}", filename!(name), index + 1, m);

                        if args.fail {
                            return Err(msg);
                        }

                        if !args.ignore {
                            eprintln!("{}", msg)
                        }
                    }
                }
            }
        }
    }

    versions.sort();

    if args.uniq {
        versions.dedup();
    }

    if args.reverse {
        versions.reverse();
    }

    for version in versions.iter() {
        println!("{}", version);
    }

    Ok(())
}
