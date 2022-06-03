#!/bin/bash

set -e

cat Cargo.toml | grep "^version = \".*\"$" | sed 's/^version = "\(.*\)"$/\1/'
