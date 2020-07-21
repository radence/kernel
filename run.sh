#!/bin/sh
cargo bootimage --target x86_64_radence.json -Z build-std -Z build-std=core,compiler_builtins,alloc
qemu-system-x86_64 -drive format=raw,file=target/x86_64_radence/debug/bootimage-radence.bin