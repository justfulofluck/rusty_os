#![no_std]
#![no_main]

mod task;

use core::default::Default;
use esp_backtrace as _;
use esp_hal::prelude::*;
use esp_println::println;
use task::TaskControlBlock;

static mut TASK1_STACK: [u8; 2048] = [0; 2048];
static mut TASK2_STACK: [u8; 2048] = [0; 2048];

fn task_one() -> ! {
    println!("TASK ONE RUNING");
    loop {}
}

fn task_two() -> ! {
    println!("TASK TWO RUNING");
    loop {}
}

#[entry]
fn main() -> ! {
    let _peripherals = esp_hal::init(esp_hal::Config::default());

    println!("=========================================");
    println!("  RustyOS: RISC-V Bare-Metal Initialized ");
    println!("  Phase 1: Toolchain & Build Succeeded!  ");
    println!("=========================================");

    // 1 TASK INISIATE
    let stack_top = unsafe { TASK1_STACK.as_ptr().add(TASK1_STACK.len()) as usize };
    let tcb1 = TaskControlBlock::new(1, 1, stack_top, task_one);

    println!("Task 1 Created | ID: {} | SP: {:#x}", tcb1.id, tcb1.sp);

    // 2 TASK inisiate
    println!("=========================================");
    println!("  Phase 2: Context Switch Succeeded!  ");
    println!("=========================================");

    let stack2_top = unsafe {
        core::ptr::addr_of_mut!(TASK2_STACK) as usize + core::mem::size_of_val(&TASK2_Stack)
    };

    loop {}
}
