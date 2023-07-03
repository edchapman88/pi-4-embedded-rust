// 1. The kernel's entry point is the function `cpu::boot::arch_boot::_start()`.
//     - It is implemented in `src/_arch/__arch_name__/cpu/boot.s`.
// 2. Once finished with architectural setup, the arch code calls `kernel_init()`.

#![feature(asm_const)]
#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod panic_wait;

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
unsafe fn kernel_init() -> ! {
    panic!()
}
