#!/usr/bin/env bash

# Exit if anything fails...
set -e

#
# Dependencies
#
cargo install xargo

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
