use std::io::{self};

use semver::{Version, VersionReq};

use clap::{IntoApp, Parser};
use clap_complete::{generate, shells::Bash};

/// Print, filter, sort lines that match a semantic version (https://semver.org).
///
/// Print lines that match a semantic version to standard output. With no FILE, or when FILE is '-', read lines from standard input.
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

    /// Filter versions according to expression. Has no meaning with --invert
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

fn filter(req: &Option<VersionReq>, ver: &Version) -> bool {
    match req {
        Some(r) => r.matches(ver),
        None => true,
    }
}

fn main() -> Result<(), String> {
    let args = Args::parse();
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
