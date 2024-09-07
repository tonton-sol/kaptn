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

    # Update dependencies on Kaptn crates in the `[dependencies]`, `[dev-dependencies]`, and `[build-dependencies]` sections
    sed "${sedi[@]}" -e "/\[dependencies\]/,/^\[/ s/\(kaptn-[a-zA-Z0-9_-]*\) = { version = \".*\"/\1 = { version = \"$version\"/" $toml
    sed "${sedi[@]}" -e "/\[dev-dependencies\]/,/^\[/ s/\(kaptn-[a-zA-Z0-9_-]*\) = { version = \".*\"/\1 = { version = \"$version\"/" $toml
    sed "${sedi[@]}" -e "/\[build-dependencies\]/,/^\[/ s/\(kaptn-[a-zA-Z0-9_-]*\) = { version = \".*\"/\1 = { version = \"$version\"/" $toml

    # Also catch cases where dependencies are specified without `{ version = ... }` syntax (just with a version)
    sed "${sedi[@]}" -e "/\[dependencies\]/,/^\[/ s/\(kaptn-[a-zA-Z0-9_-]*\) = \".*\"/\1 = \"$version\"/" $toml
    sed "${sedi[@]}" -e "/\[dev-dependencies\]/,/^\[/ s/\(kaptn-[a-zA-Z0-9_-]*\) = \".*\"/\1 = \"$version\"/" $toml
    sed "${sedi[@]}" -e "/\[build-dependencies\]/,/^\[/ s/\(kaptn-[a-zA-Z0-9_-]*\) = \".*\"/\1 = \"$version\"/" $toml
done

# Update the VERSION file
echo $version > VERSION

echo "Version updated to $version in Kaptn crates and VERSION file."
