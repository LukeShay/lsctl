#!/bin/bash

set -e

echo "Running acceptance tests"

cd npm

npm ci --no-progress --ignore-scripts

cargo run -- js config

npm run check

current_version="$(./scripts/current_version.sh)"

git tag "v${current_version}"
git push origin "v${current_version}"
