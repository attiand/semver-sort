use std::io::{self};
use std::process;

use semver::{Version, VersionReq};

use clap::{CommandFactory, Parser, ValueEnum};
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
    #[clap(short, long, num_args(1), value_name("EXPR"), value_hint = clap::ValueHint::Other)]
    filter: Option<String>,

    /// Invert match, i.e. print lines that not match a semantic version
    #[clap(short, long)]
    invert: bool,

    /// Generate completion for the specified shell and exit
    #[clap(long, num_args(1), value_name("SHELL"))]
    completion: Option<Shell>,

    /// Output format
    #[clap(long, value_enum, default_value = "default")]
    format: OutputFormat,

    /// Only print the count of matching versions
    #[clap(short, long)]
    count: bool,

    /// Suppress error messages
    #[clap(short, long)]
    quiet: bool,

    /// Files to process, if '-' read standard input
    #[clap(value_hint = clap::ValueHint::FilePath)]
    files: Vec<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    /// Default output format (version only)
    Default,
    /// JSON output format
    Json,
    /// CSV output format
    Csv,
}

fn filter(req: &Option<VersionReq>, ver: &Version) -> bool {
    match req {
        Some(r) => r.matches(ver),
        None => true,
    }
}

fn print_error(error: &str, quiet: bool) {
    if !quiet {
        eprintln!("Error: {}", error);
    }
}

fn main() {
    if let Err(e) = run() {
        print_error(&e, Args::parse().quiet);
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args = Args::parse();

    if let Some(shell) = args.completion {
        let cmd = &mut Args::command();
        generate(
            shell,
            cmd,
            Args::command().get_bin_name().unwrap_or("semver"),
            &mut io::stdout(),
        );
        return Ok(());
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
        f => version::from_files(&f),
    };

    let mut versions = match result {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    if !args.invert {
        versions.retain(|c| matches!(c, version::VersionEntry::SemVersion(v) if filter(&requirement, v)))
    } else {
        versions.retain(|c| matches!(c, version::VersionEntry::Unknown(_)))
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

    if args.count {
        println!("{}", versions.len());
        return Ok(());
    }

    match args.format {
        OutputFormat::Default => {
            for version in versions {
                println!("{version}");
            }
        }
        OutputFormat::Json => {
            println!("[");
            for (i, version) in versions.iter().enumerate() {
                if i > 0 {
                    print!(",");
                }
                match version {
                    version::VersionEntry::SemVersion(v) => println!("  {{\"version\": \"{v}\"}}"),
                    version::VersionEntry::Unknown(s) => println!("  {{\"unknown\": \"{s}\"}}"),
                }
            }
            println!("]");
        }
        OutputFormat::Csv => {
            println!("type,value");
            for version in versions {
                match version {
                    version::VersionEntry::SemVersion(v) => println!("version,{v}"),
                    version::VersionEntry::Unknown(s) => println!("unknown,{s}"),
                }
            }
        }
    }

    Ok(())
}
