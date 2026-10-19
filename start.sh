#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

./build.sh

exec qemu-system-x86_64 \
    -cdrom build/rOSt.iso \
    -m 512M \
	-display gtk \
    -no-reboot \
    -no-shutdown
