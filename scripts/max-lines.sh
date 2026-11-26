#!/bin/sh
# The length limit of source files: no Rust file is longer than 400 lines. Split a longer one into modules.
# Run by `make check`, the pre-commit hook and the release workflow.
limit=400
long=$(find src -name '*.rs' -exec wc -l {} + | awk -v limit="$limit" '$2 != "total" && $1 > limit { print "  " $2 ": " $1 " lines" }')
if [ -n "$long" ]; then
    echo "max-lines: these files are longer than $limit lines:" >&2
    echo "$long" >&2
    exit 1
fi
