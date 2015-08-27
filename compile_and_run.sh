#!/bin/bash

set -e

(cd passert_macros && cargo build)
cargo build
(cd passert_test && cargo rustc -- -Z unstable-options --pretty=expanded)
(cd passert_test && cargo test -- --test --nocapture)
