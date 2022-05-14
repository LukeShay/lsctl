#!/bin/bash

EXECUTABLE="${EXECUTABLE:-"${PWD}/target/release/lsctl"}"

BASE_PATH="target/output/fly/config"

run() {
  eval "${EXECUTABLE} ${@}"
}

rm -rf target/output/fly/config

run "fly config new --file-name ${BASE_PATH}/config.json --name the-name --organization the-org"

cat "${BASE_PATH}/config.json" &> /dev/null

run "fly config schema --file-name ${BASE_PATH}/schema.json"

cat "${BASE_PATH}/schema.json" &> /dev/null
