#!/bin/sh
set -ex
cargo test --workspace --all-features
cargo doc --workspace --all-features
cd proc-util-macros
cargo publish
cd ..
cargo publish
echo "done"
