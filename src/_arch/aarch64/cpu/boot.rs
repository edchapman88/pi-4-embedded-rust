// Architechture specific boot code
// (arch agnostic boot code is found in crate::cpu::boot::arch_boot)

use core::arch::global_asm;

// Assembly counterpart to this file.
global_asm!(include_str!("boot.s"));
