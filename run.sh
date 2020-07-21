#!/bin/sh
cargo bootimage --target *.json -Z build-std -Z build-std=core,compiler_builtins,alloc
qemu-system-x86_64 -drive format=raw,file=target/x86_64_carnel/debug/bootimage-carnel.bin