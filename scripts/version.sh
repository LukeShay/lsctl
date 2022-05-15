#!/bin/bash

set -e

sed -i.bak -E "s/^version = .+$/version = \"${1}\"/" Cargo.toml

git tag "${1}"

git push --tags
