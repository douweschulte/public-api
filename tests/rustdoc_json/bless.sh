#!/usr/bin/env bash
set -o nounset -o pipefail -o errexit

for input in syntect-v4.6.0 thiserror-v1.0.30; do
    printf "%s" "$(cargo run tests/rustdoc_json/${input}_FORMAT_VERSION_10.json)" >! "tests/rustdoc_json/${input}-expected.txt"
done
