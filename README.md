# semver
Print and optional sort lines that match a semantic version.

```
semver 2.0.0
Print and optional sort lines that match a semantic version.

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

    -h, --help
            Print help information

    -i, --invert
            Invert match, print unrecognized lines

    -r, --reverse
            Sort lines, reversed order

    -s, --sort
            Sort lines

    -u, --uniq
            Removes repeated versions (implies --sort)

    -V, --version
            Print version information
```
