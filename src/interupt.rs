use riscv::register::{scause, sepc, sscratch, stvec};

pub fn init() {
    unsafe {
        sscratch::write(0);
        stvec::write(trap_handler as usize, stvec::TrapMode::Direct);
    }
}

fn trap_handler() -> ! {
    let cause = scause::read().cause();
    let epc = sepc::read();

    println!("trap: cause: {:?}, epc: 0x{:#x}", cause, epc);
    panic!("trap handled!");
}
