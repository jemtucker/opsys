#!/usr/bin/env bash

# Exit if anything fails...
set -e

# Install rust source
RUSTUP=sh ~/rust-installer/rustup.sh
$RUSTUP component add rust-src

# Kernel
make xargo

#
# Run Tests
#

# alloc_opsys
pushd libs/alloc_opsys
cargo test
popd
