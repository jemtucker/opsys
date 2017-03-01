#!/usr/bin/env bash

# Exit if anything fails...
set -e

# Build Kernel
make xargo

# TEST: alloc_opsys
pushd libs/alloc_opsys
cargo test
popd
