#!/usr/bin/env bash

# Exit if anything fails...
set -e

# Kernel
make xargo

#
# Run Tests
#

# alloc_opsys
pushd libs/alloc_opsys
cargo test
popd
