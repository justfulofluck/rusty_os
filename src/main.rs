// src/main.rs
#![no_std]
#![no_main]

mod scheduler;
mod task;
mod timer;

use core::default::Default;
use esp_backtrace as _;
use esp_hal::prelude::*;
use esp_println::println;
use scheduler::SCHEDULER;
use task::TaskControlBlock;

static mut TASK1_STACK: [u8; 2048] = [0; 2048];
static mut TASK2_STACK: [u8; 2048] = [0; 2048];

pub static mut TASK1_RUN_COUNT: u32 = 0;
pub static mut TASK2_RUN_COUNT: u32 = 0;

fn task_one() -> ! {
    loop {
        unsafe {
            TASK1_RUN_COUNT = TASK1_RUN_COUNT.wrapping_add(1);
        }
        for _ in 0..50_000 {
            core::hint::black_box(());
        }
    }
}

fn task_two() -> ! {
    loop {
        unsafe {
            TASK2_RUN_COUNT = TASK2_RUN_COUNT.wrapping_add(1);
        }
        for _ in 0..50_000 {
            core::hint::black_box(());
        }
    }
}

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    println!("=========================================");
    println!("  RustyOS: Phase 4 Preemption Engine     ");
    println!("=========================================");

    let stack1_top = core::ptr::addr_of_mut!(TASK1_STACK) as usize + 2048;
    let stack2_top = core::ptr::addr_of_mut!(TASK2_STACK) as usize + 2048;

    let tcb1 = TaskControlBlock::new(1, 1, stack1_top, task_one);
    let tcb2 = TaskControlBlock::new(2, 1, stack2_top, task_two);

    unsafe {
        let sched = &raw mut SCHEDULER;
        (*sched).add_task(tcb1);
        (*sched).add_task(tcb2);
    }

    timer::init_systick(peripherals.TIMG0);

    unsafe {
        esp_hal::riscv::interrupt::enable();
    }

    println!("Timer Armed! Starting Preemptive Multitasking...");

    unsafe {
        let sched = &raw mut SCHEDULER;
        (*sched).start();
    }

    loop {}
}
