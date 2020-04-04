use crate::context::TrapFrame;
use riscv::register::{scause, sepc, sscratch, sstatus, stvec};

global_asm!(include_str!("trap/trap.asm"));

pub fn init() {
    unsafe {
        extern "C" {
            fn __alltraps();
        }

        sscratch::write(0);
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);

        sstatus::set_sie();
        println!("++++ setup interrupt! ++++");
    }
}

#[no_mangle]
pub fn rust_trap(tf: &mut TrapFrame) {
    println!("rust_trap!");

    tf.sepc += 2;
}
