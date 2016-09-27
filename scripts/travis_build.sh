#!/usr/bin/env bash

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
