#!/bin/bash

set -e

current_version="$(./scripts/current_version.sh)"

git tag "v${current_version}"
git push origin "v${current_version}"
