# x64

[![Crates.io](https://img.shields.io/crates/v/x64.svg)](https://crates.io/crates/x64)
[![Documentation](https://docs.rs/x64/badge.svg)](https://docs.rs/x64)
[![License](https://img.shields.io/crates/l/x64.svg)](LICENSE-MIT)

**x86_64 support with LA57 (5-level paging) for 57-bit virtual addresses.**

> *"The future is already here — it's just not evenly distributed."* — William Gibson

This crate is a fork of the excellent [rust-osdev/x86_64](https://github.com/rust-osdev/x86_64), adding runtime support for **5-level paging (LA57)**, which enables:

- **57-bit virtual addresses** (vs 48-bit in 4-level paging)
- **128 PB addressable memory** (vs 256 TB)
- **Future-proof OS development** for next-generation hardware

---

## Why This Fork Exists

### The Problem

The upstream x86_64 crate (as of 0.15.x) has **hardcoded 48-bit canonical address validation**:

```rust
// In x86_64 crate's addr.rs
pub const fn new_truncate(addr: u64) -> VirtAddr {
    VirtAddr(((addr << 16) as i64 >> 16) as u64)  // Always assumes 48-bit!
}
```

This causes panics when LA57 (5-level paging) is enabled:

```
panicked at 'virtual address must be sign extended in bits 48 to 64'
```

Addresses like `0x0000_8000_0000_0000` are **perfectly valid** in LA57 mode (where bit 56 is the sign bit, not bit 47), but the x86_64 crate rejects them.

### The Solution

**x64** provides runtime detection of the paging mode:

```rust
use x64::{VirtAddr, enable_la57_mode, is_la57_mode};

// During early boot, after enabling LA57 in CR4:
enable_la57_mode();

// Now addresses work correctly for 5-level paging:
let addr = VirtAddr::new(0x0000_8000_0000_0000);  // Works!

// Check current mode
if is_la57_mode() {
    println!("Using 57-bit virtual addresses (128 PB addressable)");
}
```

---

## Paging Mode Comparison

| Feature | 4-Level Paging | 5-Level Paging (LA57) |
|:--------|:--------------:|:---------------------:|
| Virtual Address Bits | 48 | **57** |
| Sign Extension Bit | 47 | **56** |
| Addressable Memory | 256 TB | **128 PB** |
| Page Table Levels | 4 (PML4) | **5 (PML5)** |
| CPU Requirement | Standard x86_64 | Ice Lake+ / Zen 4+ |

### Canonical Address Rules

- **4-level paging**: Bits 48-63 must be copies of bit 47
- **5-level paging**: Bits 57-63 must be copies of bit 56

---

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
x64 = "0.16"
```

### Basic Example

```rust
use x64::{VirtAddr, PhysAddr, enable_la57_mode, is_la57_mode};

fn init_paging() {
    // Check if LA57 is enabled in CR4 (you'd read CR4 in your kernel)
    let cr4_la57_enabled = /* read CR4 bit 12 */;

    if cr4_la57_enabled {
        enable_la57_mode();
        println!("5-level paging active: 57-bit virtual addresses");
    }

    // VirtAddr::new() now validates correctly for the active mode
    let kernel_base = VirtAddr::new(0xFFFF_8000_0000_0000);
}
```

### Page Table Index Methods

```rust
use x64::VirtAddr;

let addr = VirtAddr::new(0xFFFF_8000_1234_5678);

// Standard 4-level paging indices
let p1 = addr.p1_index();  // Level 1 (PT)
let p2 = addr.p2_index();  // Level 2 (PD)
let p3 = addr.p3_index();  // Level 3 (PDPT)
let p4 = addr.p4_index();  // Level 4 (PML4)

// NEW: 5-level paging index (LA57 only)
let p5 = addr.p5_index();  // Level 5 (PML5)
```

---

## API Compatibility

This crate maintains **full API compatibility** with rust-osdev/x86_64, plus LA57 extensions:

| Function | Description |
|:---------|:------------|
| `enable_la57_mode()` | Switch to 57-bit address validation |
| `disable_la57_mode()` | Switch to 48-bit address validation (default) |
| `is_la57_mode()` | Check current validation mode |
| `VirtAddr::p5_index()` | Level 5 page table index (LA57) |
| `VirtAddr::new_const()` | Const-compatible constructor (assumes 48-bit) |

---

## Migration from x86_64

```diff
- [dependencies]
- x86_64 = "0.15"
+ [dependencies]
+ x64 = "0.16"
```

```diff
- use x86_64::{VirtAddr, PhysAddr};
+ use x64::{VirtAddr, PhysAddr, enable_la57_mode};

fn init() {
+   // If using LA57, enable it:
+   if cr4_has_la57() {
+       enable_la57_mode();
+   }

    // Everything else works the same!
    let addr = VirtAddr::new(0x1000);
}
```

---

## Acknowledgments

This crate stands on the shoulders of giants.

### Special Thanks

**[Philipp Oppermann](https://github.com/phil-opp)** — For creating the original x86_64 crate and his incredible [Writing an OS in Rust](https://os.phil-opp.com/) blog series, which has taught thousands of developers how to build operating systems. His work is foundational to the Rust OS development ecosystem.

### Original Authors

- [Philipp Oppermann](https://github.com/phil-opp)
- [Gerd Zellweger](https://github.com/gz)
- [Eric Kidd](https://github.com/emk)
- [Dan Schatzberg](https://github.com/dschatzberg)
- [John Ericson](https://github.com/Ericson2314)
- [Rex Lunae](https://github.com/rexlunae)

And all [contributors](https://github.com/rust-osdev/x86_64/graphs/contributors) to the rust-osdev/x86_64 project.

---

## Why "x64"?

Because sometimes the best names are the simplest. This crate is `x64` — x86_64 with LA57 support, ready for the next 64 petabytes of memory.

---

## Crate Feature Flags

Inherited from the original x86_64 crate:

* `nightly`: Enables features only available on nightly Rust; enabled by default.
* `instructions`: Enabled by default, turns on x86_64 specific instructions. Only available for x86_64 targets.

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Same license as the original x86_64 crate.

---

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

---

*Built for [LunaOS](https://github.com/artst3in/LunaOS) — A sovereign operating system with LA57 support since day 1.*
