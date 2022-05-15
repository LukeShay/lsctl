#!/bin/bash

set -e

TAG="$(cat Cargo.toml | grep "^version = .*" | sed -E "s/^version = \"(.*)\"$/\1/")"

git tag "${TAG}"
git push origin "${TAG}"
