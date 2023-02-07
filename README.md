# semver
Print, filter, sort lines that match a semantic version (https://semver.org)

Internally uses https://crates.io/crates/semver for sorting and matching. Filter expressions syntax is described [here](https://docs.rs/semver/1.0.9/semver/struct.VersionReq.html#syntax)

```
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
            Filter versions according to expression. Has no meaning with --invert

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

Print git tags that matches a semantic version.

```bash
git tag | semver
```

Print the highest sematic version in current git repo.

```bash
git tag | semver --sort | tail -n1
```

Print lines that matches a semantic version and has major version number 1 from specified file.

```bash
semver --filter '>= 1, <2' tags.txt
```

Print all versions between `1.2.0` and `1.3.7` (inclusive) from specified files.

```bash
semver --filter '>= 1.2.0, <=1.3.7' tags1.txt tags2.txt
```
