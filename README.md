# semver
Print and optional sort lines that match a semantic version.

```
semver 2.0.0-beta.2
Print and optional sort lines that match a semantic version.

Print semantic versions to standard output. With no FILE, or when FILE is '-', read standard input.

USAGE:
    semver [OPTIONS] [FILES]...

ARGS:
    <FILES>...
            Files to process, if '-' read standard input

OPTIONS:
        --completion
            Generate bash completion and exit

    -f, --fail
            Fail for any unrecognized versions

    -h, --help
            Print help information

    -r, --reverse
            Reverse the result of comparisons (implies --sort)

    -s, --sort
            Sort lines

    -u, --uniq
            Removes repeated versions (implies --sort)

    -V, --version
            Print version information
```
