#!/bin/bash
# Tại thư mục rust_api
cargo sqlx prepare

cargo clippy --all-targets --all-features -- -D warnings