# semver-sort
Sort lines of text files according to semantic versioning

```
semver-sort 0.7.0
Sort lines of text files according to semantic versioning.

Write sorted lines to standard output. With no FILE, or when FILE is '-', read standard input.

USAGE:
    semver-sort [OPTIONS] [FILES]...

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

    -i, --ignore
            Silently ignore lines with unrecognized versions

    -r, --reverse
            Reverse the result of comparisons

    -V, --version
            Print version information
```
