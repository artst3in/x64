//! # x64
//!
//! x86_64 support with **LA57 (5-level paging)** for 57-bit virtual addresses.
//!
//! This crate is a fork of the excellent [rust-osdev/x86_64](https://github.com/rust-osdev/x86_64)
//! crate, adding runtime support for 5-level paging (LA57) which enables 57-bit virtual addresses
//! and up to 128 PB of addressable memory.
//!
//! ## The Problem
//!
//! The upstream x86_64 crate (as of 0.15.x) has hardcoded 48-bit canonical address validation.
//! When LA57 is enabled in CR4, addresses like `0x0000_8000_0000_0000` are perfectly valid
//! (bit 56 is the sign bit, not bit 47), but the crate rejects them:
//!
//! ```text
//! panicked at 'virtual address must be sign extended in bits 48 to 64'
//! ```
//!
//! ## The Solution
//!
//! This crate provides runtime detection of the paging mode:
//!
//! ```ignore
//! use x64::{VirtAddr, addr::{enable_la57_mode, is_la57_mode}};
//!
//! // During early boot, after enabling LA57 in CR4:
//! enable_la57_mode();
//!
//! // Now addresses work correctly for 5-level paging:
//! let addr = VirtAddr::new(0x0000_8000_0000_0000);  // Works in LA57!
//! ```
//!
//! ## Credits
//!
//! This crate builds on the incredible work of [Philipp Oppermann](https://github.com/phil-opp)
//! and all contributors to the [rust-osdev/x86_64](https://github.com/rust-osdev/x86_64) project.
//! We are deeply grateful for their foundational contributions to OS development in Rust.

#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "abi_x86_interrupt", feature(abi_x86_interrupt))]
#![cfg_attr(feature = "step_trait", feature(step_trait))]
#![cfg_attr(feature = "doc_cfg", feature(doc_cfg))]
#![warn(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(unsafe_op_in_unsafe_fn)]

// Core address types and LA57 support
pub use crate::addr::{
    align_down, align_up,
    enable_la57_mode, disable_la57_mode, is_la57_mode, LA57_ENABLED,
    PhysAddr, VirtAddr,
};

pub mod addr;
pub mod instructions;
pub mod registers;
pub mod structures;

/// Represents a protection ring level.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PrivilegeLevel {
    /// Privilege-level 0 (most privilege): This level is used by critical system-software
    /// components that require direct access to, and control over, all processor and system
    /// resources. This can include BIOS, memory-management functions, and interrupt handlers.
    Ring0 = 0,

    /// Privilege-level 1 (moderate privilege): This level is used by less-critical system-
    /// software services that can access and control a limited scope of processor and system
    /// resources. Software running at these privilege levels might include some device drivers
    /// and library routines. The actual privileges of this level are defined by the
    /// operating system.
    Ring1 = 1,

    /// Privilege-level 2 (moderate privilege): Like level 1, this level is used by
    /// less-critical system-software services that can access and control a limited scope of
    /// processor and system resources. The actual privileges of this level are defined by the
    /// operating system.
    Ring2 = 2,

    /// Privilege-level 3 (least privilege): This level is used by application software.
    /// Software running at privilege-level 3 is normally prevented from directly accessing
    /// most processor and system resources. Instead, applications request access to the
    /// protected processor and system resources by calling more-privileged service routines
    /// to perform the accesses.
    Ring3 = 3,
}

impl PrivilegeLevel {
    /// Creates a `PrivilegeLevel` from a numeric value. The value must be in the range 0..4.
    ///
    /// This function panics if the passed value is >3.
    #[inline]
    pub const fn from_u16(value: u16) -> PrivilegeLevel {
        match value {
            0 => PrivilegeLevel::Ring0,
            1 => PrivilegeLevel::Ring1,
            2 => PrivilegeLevel::Ring2,
            3 => PrivilegeLevel::Ring3,
            _ => panic!("invalid privilege level"),
        }
    }
}

pub(crate) mod sealed {
    pub trait Sealed {}
}
