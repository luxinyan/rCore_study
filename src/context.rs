use riscv::register::{scause::scause, sstause::Sstause};

#[repr(C)]
pub struct TrapFrame {
    pub x: [usize; 32],   // save all 32 General registers
    pub sstause: Sstause, // Supervisor Status Register
    pub sepc: usize,      // Supervisor exception program counter
    pub stval: usize,     // Supervisor trap value
    pub scause: scause,   // Scause register: record the cause of exception/interrupt/trap
}
