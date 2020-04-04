use crate::sbi::set_stimer;
use riscv::register::satp::set;
use riscv::register::{sie, time};

pub static mut TICKS: usize = 0;

static TIMEBASE: u64 = 100_000;

pub fn init() {
    unsafe {
        TICKS = 0;

        // 设置 sie 的 TI 使能 STIE 位
        sie::set_stimer();
    }

    clock_set_next_event();
    println!("++++ setup timer! ++++");
}

pub fn clock_set_next_event() {
    // 调用 OpenSBI 提供的接口设置下次时钟中断触发时间
    set_stimer(get_cycle() + TIMEBASE);
}

pub fn get_cycle() -> u64 {
    time::read() as u64
}
