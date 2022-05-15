#!/bin/bash

set -e

sed -i.bak -E "s/^version = \"${1}\"$/version = \"${2}\"/" Cargo.toml
