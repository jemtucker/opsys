#!/usr/bin/env bash

# Exit if anything fails...
# This is temporarily disabled as rustfmt has not yet been updated to handle
# the new 'x86-interrupt' ABI. 
# set -e

# If this is not a PR then there is nothing to do
if [ "$TRAVIS_PULL_REQUEST" == "false" ]
then
    exit 0
fi

# Write rustfmt version for test
cargo fmt -- --version

# Run format checks
cargo fmt -- --write-mode=diff

# Exit success always for now (see comment above)
exit 0
