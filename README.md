# semver
Print, filter, sort lines that match a semantic version (https://semver.org)

Internally uses https://crates.io/crates/semver for sorting and matching. Filter expressions syntax is described [here](https://docs.rs/semver/1.0.9/semver/struct.VersionReq.html#syntax)

```
semver 2.1.0
Print, filter, sort lines that match a semantic version (https://semver.org).

Print lines that match a semantic version to standard output. With no FILE, or when FILE is '-',
read standard input.

USAGE:
    semver [OPTIONS] [FILES]...

ARGS:
    <FILES>...
            Files to process, if '-' read standard input

OPTIONS:
        --completion
            Generate bash completion and exit

    -f, --filter <EXPR>
            Filter versions acording to expression. Has no meaning with --invert

    -h, --help
            Print help information

    -i, --invert
            Invert match, i.e. print lines that not match a semantic version

    -r, --reverse
            Sort lines in reversed order

    -s, --sort
            Sort lines

    -u, --uniq
            Removes repeated versions (implies --sort)

    -V, --version
            Print version information
```

## Examples

Print tags that matches semantic version.

> git tag | semver

Print tags that has major version number 1. 

> semver --filter '>= 1.0.0, <2.0.0' tags.txt