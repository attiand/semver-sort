# semver
Filter or sort lines of files according to semantic versioning.

```
semver 2.0.0
Filter or sort lines of files according to semantic versioning.

Print semantic versions to standard output. With no FILE, or when FILE is '-', read standard input.

USAGE:
    semver [OPTIONS] [FILES]...

ARGS:
    <FILES>...
            Files to sort, if '-' read standard input

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
