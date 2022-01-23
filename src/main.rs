use std::fs;
use std::process;
use std::io::{self, BufReader, BufRead};

use semver::Version;

use clap::{Parser, IntoApp};

use clap_complete::{generate, shells::Bash};

/// Sort lines of text files according to semantic versioning.
/// 
/// Write sorted lines to standard output. With no FILE, or when FILE is -, read standard input.
#[derive(Parser)]
#[clap(author,
    version,
    about,
)]

struct Args {

    /// Reverse the result of comparisons
    #[clap(short, long, takes_value = false)]
    reverse: bool,

    /// Silently ignore lines with unrecognized versions
    #[clap(short, long, takes_value = false)]
    ignore: bool,

    /// Fail for lines with unrecognized versions
    #[clap(short, long, takes_value = false)]
    fail: bool,

    /// File to sort
    file: Option<String>,

    /// Generate bash completion and exit
    #[clap(long, takes_value = false)]
    completion: bool,
}

fn main() {
    let args = Args::parse();

    let mut app = Args::into_app();

    if args.completion {
        generate(Bash, &mut app, "semver-sort", &mut io::stdout());
        process::exit(1)
    }

    let reader: Box<dyn BufRead> = match args.file {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) if filename == "-" => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(fs::File::open(&filename).unwrap_or_else(|e| {
            eprintln!("{}, {}", filename, e);
            process::exit(1)
        })))
    };

    let mut versions = Vec::new();

    for line in reader.lines() {
        if let Ok(l) = line {
            let version = Version::parse(&l);
            match version {
                Ok(v) => {
                    versions.push(v);
                },
                Err(m) => {
                    if! args.ignore {
                        eprintln!("Not a semantic version: '{}', {}", l, m) 
                    }

                    if args.fail {
                        process::exit(2)
                    }
                },
            }
        }
    }

    versions.sort();

    if args.reverse {
        versions.reverse();
    }

    for version in versions.iter() {
        println!("{}", version.to_string());
    }
}
