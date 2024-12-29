use std::io::{self};

use semver::{Version, VersionReq};

use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};

/// Print, filter, sort lines that match a semantic version (https://semver.org).
///
/// Print lines that match a semantic version to standard output. With no FILE, or when FILE is '-', read lines from standard input.
#[derive(Parser)]
#[clap(author, version, about)]
struct Args {

    /// Sort lines
    #[clap(short, long)]
    sort: bool,

    /// Sort lines in reversed order
    #[clap(short, long)]
    reverse: bool,

    /// Removes repeated versions (implies --sort)
    #[clap(short, long)]
    uniq: bool,

    /// Filter versions according to expression. Has no meaning with --invert
    #[clap(short, long, num_args(1), value_name("EXPR"))]
    filter: Option<String>,

    /// Invert match, i.e. print lines that not match a semantic version
    #[clap(short, long)]
    invert: bool,

    /// Generate completion for the specified shell and exit
    #[clap(long, num_args(1), value_name("SHELL"))]
    completion: Option<Shell>,

    /// Files to process, if '-' read standard input
    files: Vec<String>,
}

fn filter(req: &Option<VersionReq>, ver: &Version) -> bool {
    match req {
        Some(r) => r.matches(ver),
        None => true,
    }
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    match args.completion {
        Some(shell) => {
            let mut cmd = Args::command();
            generate(shell, &mut cmd, "semver", &mut io::stdout());
            return Ok(());
        },
        None => (),
    }

    let requirement = match args.filter {
        Some(f) => match VersionReq::parse(&f) {
            Ok(r) => Some(r),
            Err(e) => return Err(format!("Illegal filter format expression: {e}")),
        },
        None => None,
    };

    let result = match args.files {
        f if f.is_empty() => version::from_stdin(),
        f if f.len() == 1 && args.files.first().unwrap().eq("-") => version::from_stdin(),
        f => version::from_files(f.iter().map(String::as_ref).collect()),
    };

    let mut versions = match result {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    if !args.invert {
        versions.retain(|c| matches!(c, version::Type::SemVersion(v) if filter(&requirement, v)))
    } else {
        versions.retain(|c| matches!(c, version::Type::Unknown(_)))
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
        println!("{version}")
    }

    Ok(())
}
