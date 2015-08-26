#!/bin/bash

set -e

(cd passert_macros && cargo build)
(cd passert_test && cargo rustc -- -Z unstable-options --pretty=expanded)
(cd passert_test && cargo test)
