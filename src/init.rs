global_asm!(include_str!("boot/entry64.asm"));

#[no_mangle]
extern "C" fn rust_main() -> ! {
    crate::interrupt::init();

    unsafe {
        asm!("ebreak"
              :
              :
              :
              "volatile");
    }

    panic!("end of rust_main");

    loop {}
}
