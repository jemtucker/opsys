#!/usr/bin/env bash

# TODO Take this out its just to work out whats going on...
pwd

# Exit if anything fails...
set -e

#
# Build
#

# Kernel
make cargo

#
# Run Tests
#

# alloc_opsys
pushd libs/alloc_opsys
cargo test
popd
