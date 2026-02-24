#!/usr/bin/bash
set -e

print_header () {
    echo -e "\e[4;96m$1\e[0m"
}

print_header "Running Checks"
cargo check --quiet

print_header "Running Tests"
cargo test -rq --tests

TARGETS="x86_64-unknown-linux-gnu x86_64-pc-windows-gnu"
for t in $TARGETS; do
    print_header "Building Target \e[1m$t"
    cargo build --release --target $t
    echo
done
