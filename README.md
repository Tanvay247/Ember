# Ember

A high-performance Rust unikernel designed for ultra-fast boot times,
minimal memory overhead, and hyper-dense cloud execution.

## Goals

- Sub-10ms boot
- Single address space architecture
- Zero-copy networking
- Virtio-based microVM execution
- Rust memory safety
- Firecracker compatibility

## Current Status

- [x] Bare-metal kernel foundation
- [x] Custom x86_64 target
- [x] Serial output
- [x] QEMU boot
- [ ] Memory management
- [ ] Interrupt handling
- [ ] Heap allocator
- [ ] Async executor
- [ ] Virtio networking