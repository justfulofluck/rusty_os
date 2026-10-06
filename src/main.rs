#![no_std]
#![no_main]

mod scheduler;
mod task;

use esp_backtrace as _;
use esp_hal::prelude::*;
use esp_println::println;
use scheduler::{task_yield, SCHEDULER};
use task::{switch_context, TaskControlBlock};

static mut TASK1_STACK: [u8; 2048] = [0; 2048];
static mut TASK2_STACK: [u8; 2048] = [0; 2048];

fn task_one() -> ! {
    let mut count: u32 = 0;
    loop {
        println!("[Task 1] Running... Iteration: {}", count);
        count += 1;

        // Simulate work with bu7sy delay
        for _ in 0..1_000_000 {
            core::hint::black_box(());
        }

        println!("[Task 1] Yielding CPU to Task 2 ->");
        task_yield();
    }
}

fn task_two() -> ! {
    let mut count: u32 = 0;
    loop {
        println!("[Task 2] Running... Iteration: {}", count);
        count += 1;

        for _ in 0..1_000_000 {
            core::hint::black_box(());
        }

        println!("[Task 2] Yielding CPU to Task 1 ->")
    }
}

#[entry]
fn main() -> ! {
    let _peripherals = esp_hal::init(esp_hal::Config::default());

    println!("=========================================");
    println!("  RustyOS: Phase 3 Round-Robin Scheduler ");
    println!("=========================================");

    // 1 TASK INISIATE
    let stack1_top = core::ptr::addr_of_mut!(TASK1_STACK) as usize + 2048;
    let stack2_top = core::ptr::addr_of_mut!(TASK2_STACK) as usize + 2048;

    let tcb1 = TaskControlBlock::new(1, 1, stack1_top, task_one);
    let tcb2 = TaskControlBlock::new(2, 1, stack2_top, task_two);

    unsafe {
        let sched = &raw mut SCHEDULER;
        (*sched).add_task(tcb1);
        (*sched).add_task(tcb2);
    }

    println!("Task registered. Starting Task 1...");

    let mut main_sp: usize = 0;
    let first_task_sp = unsafe { SCHEDULER.tasks[0].as_ref().unwrap().sp };

    unsafe {
        switch_context(&raw mut main_sp, &first_task_sp);
    }

    loop {}
}
