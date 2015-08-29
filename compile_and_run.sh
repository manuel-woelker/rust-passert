#!/bin/bash

set -e

(cd passert_macros && cargo build)
cargo build
(cd passert_test && cargo rustc -- --cfg test -Z unstable-options --pretty=expanded)
(cd passert_test && RUST_BACKTRACE=1 RUST_TEST_TASKS=1 cargo test -- --test --nocapture)
