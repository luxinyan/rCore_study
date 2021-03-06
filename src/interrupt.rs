use crate::context::TrapFrame;
use riscv::register::{
    scause::{Exception, Interrupt, Trap},
    sscratch, sstatus, stvec,
};

use crate::timer::{clock_set_next_event, TICKS};

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
    match tf.scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(&mut tf.sepc),
        Trap::Exception(Exception::IllegalInstruction) => illegal_instruction(&mut tf.sepc),
        Trap::Interrupt(Interrupt::SupervisorTimer) => super_timer(),
        _ => panic!("undefined trap!"),
    }
}

fn breakpoint(sepc: &mut usize) {
    println!("a breakpoint set @0x{:x}", sepc);
    *sepc += 2;
}

fn illegal_instruction(sepc: &mut usize) {
    panic!("illegal instruction at @0x{:x}", sepc);
}

fn super_timer() {
    clock_set_next_event();
    unsafe {
        TICKS += 1;
        if TICKS == 100 {
            TICKS = 0;
            println!("* 100 ticks *");
        }
    }
}
