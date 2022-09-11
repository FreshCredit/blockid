#!/usr/bin/env bash

set -euxo pipefail

cargo fmt --all ---check
cargo check
cargo test
