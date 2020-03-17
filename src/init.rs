global_asm!(include_str!("boot/entry64.asm"));

use crate::sbi;

#[no_mangle]
extern "C" fn rust_main() -> ! {
    sbi::console_putchar(b'O' as usize);
    sbi::console_putchar(b'K' as usize);
    sbi::console_putchar(b'\n' as usize);

    loop {}
}
