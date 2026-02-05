#!/bin/bash
set -e

# Build Rust library for watchOS targets
# Output goes to target/<target>/release/libtennis_scorer.a

echo "Building tennis-scorer for watchOS..."
echo ""

# Simulator (Apple Silicon Mac)
echo "=== Building for aarch64-apple-watchos-sim (Simulator) ==="
cargo +nightly build --release --target aarch64-apple-watchos-sim

# Device (Apple Watch Series 4+)
echo ""
echo "=== Building for aarch64-apple-watchos (Device) ==="
cargo +nightly build --release --target aarch64-apple-watchos

echo ""
echo "=== Build complete ==="
echo ""
echo "Libraries:"
ls -lh target/aarch64-apple-watchos-sim/release/libtennis_scorer.a 2>/dev/null || echo "  Simulator: not found"
ls -lh target/aarch64-apple-watchos/release/libtennis_scorer.a 2>/dev/null || echo "  Device: not found"
echo ""
echo "Xcode should reference:"
echo "  Simulator: \$(PROJECT_DIR)/../target/aarch64-apple-watchos-sim/release"
echo "  Device:    \$(PROJECT_DIR)/../target/aarch64-apple-watchos/release"
