#!/usr/bin/env sh
set -eu

# Ensure Rust toolchain is on PATH for POSIX sh
export RUSTUP_HOME="/usr/local/rustup"
export CARGO_HOME="/usr/local/cargo"
export PATH="/usr/local/cargo/bin:${PATH:-/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin}"

# Quick sanity check to help diagnose PATH issues inside container
if ! command -v cargo >/dev/null 2>&1; then
  echo "Error: cargo not found in PATH: $PATH" >&2
  exit 127
fi

cargo --version

# Build Clippy SARIF report
cargo clippy --all-targets --all-features --message-format=json | clippy-sarif | tee docker-build/reports/results.sarif
cargo clippy --all-targets --all-features --message-format=json | tee docker-build/reports/results.clippy