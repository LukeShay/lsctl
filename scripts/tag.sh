#!/bin/bash

set -e

TAG="$(cat Cargo.toml | grep "^version = .*" | sed -E "s/^version = \"(.*)\"$/\1/")"

git tag "v${TAG}"
git push origin "v${TAG}"
