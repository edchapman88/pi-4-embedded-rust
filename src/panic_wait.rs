use core::panic::PanicInfo;

use crate::cpu::wait_forever;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    wait_forever()
}
