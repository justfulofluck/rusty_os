use esp_hal::prelude::_esp_hal_timer_timg_Instance;

use esp_hal::{
    interrupt::{self, Priority},
    peripherals::{Interrupt, TIMG0},
    prelude::*,
    time::Duration,
    timer::{
        timg::{Timer, Timer0, TimerGroup},
        Timer as _,
    },
    Blocking,
};

pub static mut TIMER0: Option<Timer<Timer0<TIMG0>, Blocking>> = None;
//pub static mut TICK_COUNT: u32 = 0;

pub fn init_systick(timg0_peripheral: TIMG0) {
    let timg_group = TimerGroup::new(timg0_peripheral);
    let mut timer0 = timg_group.timer0;

    // enable intruprs and put alaram of given ms
    timer0.set_interrupt_handler(timer_isr);
    timer0.listen();

    let interval = Duration::millis(100);
    timer0.load_value(interval).unwrap();
    timer0.start();

    unsafe {
        TIMER0 = Some(timer0);
    }

    // risc-v intrupt contorler
    let _ = interrupt::enable(Interrupt::TG0_T0_LEVEL, Priority::Priority1);
}

// Time ISR

#[handler(priority = esp_hal::interrupt::Priority::Priority1)]
fn timer_isr() {
    unsafe {
        //TIMER0.as_mut().unwrap().clear_interrupt();
        if let Some(ref mut timer) = TIMER0 {
            timer.clear_interrupt();

            timer.load_value(Duration::millis(100)).unwrap();
            timer.start();
        }
    }
    // premetive switch
    crate::scheduler::task_yield();
}
