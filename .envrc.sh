#!/bin/bash

# Dependencies:
# - bash
# - nix

function develop() {
  # Start Development Environment.
  nix develop \
    --experimental-features 'nix-command flakes' \
    --show-trace \
    --option max-jobs 8 \
    --option cores 2 \
    --verbose \
    --ignore-environment \
    "."
}
