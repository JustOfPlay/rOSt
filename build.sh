#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

KERNEL="kernel/target/x86_64-unknown-none/release/kernel"

echo "==> Building kernel..."
cargo build --release --manifest-path kernel/Cargo.toml \
    --target x86_64-unknown-none

if [[ ! -f "$KERNEL" ]]; then
    KERNEL="target/x86_64-unknown-none/release/kernel"
fi

if ! grub-file --is-x86-multiboot2 "$KERNEL"; then
    echo "Error: Invalid Multiboot2-Kernel: $KERNEL"
    exit 1
fi

echo "==> Preparing ISO..."
ISO_DIR="build/iso"
mkdir -p "$ISO_DIR/boot/grub"
cp "$KERNEL" "$ISO_DIR/boot/kernel"

cat > "$ISO_DIR/boot/grub/grub.cfg" <<'EOF'
set timeout=0
set default=0

menuentry "rOSt" {
    multiboot2 /boot/kernel
    boot
}
EOF

echo "==> Creating ISO..."
grub-mkrescue -o build/rOSt.iso "$ISO_DIR"

echo "==> Build successful: build/rOSt.iso"
