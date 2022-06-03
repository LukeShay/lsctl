#!/bin/bash

set -e

current_version="$(./scripts/current_version.sh)"

sed -i.bak -E "s/^version = \"${current_version}\"$/version = \"${1}\"/" Cargo.toml
sed -i.bak -E "s/\"version\": \"${current_version}\"/\"version\": \"${1}\"/" npm/package.json
