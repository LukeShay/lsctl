#!/bin/bash

EXECUTABLE="${1:-"${PWD}/target/release/lsctl"}"

BASE_PATH="target/output/fly/config"

run() {
  eval "${EXECUTABLE} ${@}"
}

rm -rf target/output/fly/config

run "fly config new --file ${BASE_PATH}/config.json --name 'the-{{ environment }}-name' --organization 'the-{{ environment }}-org'"

cat "${BASE_PATH}/config.json" &> /dev/null

run "fly config schema --file ${BASE_PATH}/schema.json"

cat "${BASE_PATH}/schema.json" &> /dev/null

run "fly config gen --input-file ${BASE_PATH}/config.json --output-file ${BASE_PATH}/fly.toml"
