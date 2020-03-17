global_asm!(include_str!("boot/entry64.asm"));

#[no_mangle]
extern "C" fn rust_main() -> ! {
    crate::interupt::init();

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
