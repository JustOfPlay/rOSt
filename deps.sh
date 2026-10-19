#!/usr/bin/env bash
set -euo pipefail

echo "==> Installing system dependencies..."

sudo pacman -S --needed \
    rustup \
    qemu-system-x86 \
    qemu-ui-gtk \
    grub \
    xorriso \
    mtools \
    binutils

echo "==> Installing Rust target..."
rustup target add x86_64-unknown-none

echo "==> Checking required commands..."

for cmd in cargo rustc qemu-system-x86_64 grub-file grub-mkrescue; do
    if ! command -v "$cmd" >/dev/null 2>&1; then
        echo "Missing command: $cmd"
        exit 1
    fi
done

echo "==> All dependencies are ready!"


