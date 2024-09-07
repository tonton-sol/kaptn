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
    sed "${sedi[@]}" -E "/\[package\]/,/^\[/ s/version = \"[^\"]+\"/version = \"$version\"/" "$toml"

    # Update dependencies on Kaptn crates in the `[dependencies]`, `[dev-dependencies]`, and `[build-dependencies]` sections
    # Preserve additional attributes like `path`, `features`, etc.
    sed "${sedi[@]}" -E "/\[dependencies\]/,/^\[/ s/(kaptn-[a-zA-Z0-9_-]*) = \{([^}]*)version = \"[^\"]+\"/\1 = {\2version = \"$version\"/" "$toml"
    sed "${sedi[@]}" -E "/\[dev-dependencies\]/,/^\[/ s/(kaptn-[a-zA-Z0-9_-]*) = \{([^}]*)version = \"[^\"]+\"/\1 = {\2version = \"$version\"/" "$toml"
    sed "${sedi[@]}" -E "/\[build-dependencies\]/,/^\[/ s/(kaptn-[a-zA-Z0-9_-]*) = \{([^}]*)version = \"[^\"]+\"/\1 = {\2version = \"$version\"/" "$toml"

    # Also handle dependencies with just the version (no additional attributes)
    sed "${sedi[@]}" -E "/\[dependencies\]/,/^\[/ s/(kaptn-[a-zA-Z0-9_-]*) = \"[^\"]+\"/\1 = \"$version\"/" "$toml"
    sed "${sedi[@]}" -E "/\[dev-dependencies\]/,/^\[/ s/(kaptn-[a-zA-Z0-9_-]*) = \"[^\"]+\"/\1 = \"$version\"/" "$toml"
    sed "${sedi[@]}" -E "/\[build-dependencies\]/,/^\[/ s/(kaptn-[a-zA-Z0-9_-]*) = \"[^\"]+\"/\1 = \"$version\"/" "$toml"
done

# Update the VERSION file
echo "$version" > VERSION

echo "Version updated to $version in Kaptn crates and VERSION file."
