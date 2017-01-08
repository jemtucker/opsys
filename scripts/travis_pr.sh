#!/usr/bin/env bash

# Exit if anything fails...
set -e

# If this is not a PR then there is nothing to do
if [ "$TRAVIS_PULL_REQUEST" == "false" ]
then
    exit 0
fi

# Run format checks
cargo fmt -- --write-mode=diff
