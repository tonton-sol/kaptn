#!/bin/bash

set -e

if [ $# -eq 0 ]; then
    echo "Usage: $0 VERSION"
    exit 1
fi

version=$1

echo "Bumping Kaptn crate versions to $version"

# GNU/BSD compatibility for `sed` across systems
sedi=(-i)
case "$(uname)" in
  Darwin*) sedi=(-i "") ;; # macOS compatibility for `sed`
esac

# Find and update Cargo.toml files only in Kaptn crates
for toml in $(find . -name "Cargo.toml"); do
    # Update the package version in the `[package]` section
    sed "${sedi[@]}" -e "/\[package\]/,/^\[/ s/version = \".*\"/version = \"$version\"/" $toml
    
    # Update dependencies on Kaptn crates in the `[dependencies]` or `[dev-dependencies]` sections
    sed "${sedi[@]}" -e "/\[dependencies\]/,/^\[/ s/kaptn-.*/kaptn-\*: \"$version\"/" $toml
    sed "${sedi[@]}" -e "/\[dev-dependencies\]/,/^\[/ s/kaptn-.*/kaptn-\*: \"$version\"/" $toml
done

echo "Version updated to $version in Kaptn crates."
