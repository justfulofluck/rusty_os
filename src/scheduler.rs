use crate::task::{switch_context, TaskControlBlock, TaskState};

const MAX_TASKS: usize = 4;

pub struct Scheduler {
    pub tasks: [Option<TaskControlBlock>; MAX_TASKS],
    current: usize,
    task_count: usize,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            tasks: [None, None, None, None],
            current: 0,
            task_count: 0,
        }
    }

    // new task scheduler
    pub fn add_task(&mut self, tcb: TaskControlBlock) -> bool {
        if self.task_count < MAX_TASKS {
            self.tasks[self.task_count] = Some(tcb);
            self.task_count += 1;
            true
        } else {
            false
        }
    }

    pub fn start(&mut self) {
        assert!(self.task_count > 0, "No tasks to register");
        self.current = 0;
        self.tasks[0].as_mut().unwrap().state = TaskState::Running;

        let next_sp = self.tasks[1].as_ref().unwrap().sp;
        let mut dummy_sp: usize = 0;

        unsafe {
            switch_context(&mut dummy_sp, &next_sp);
        }

        loop {}
    }

    //  to find next ready task for round robin
    pub fn schedule_next(&mut self) {
        if self.task_count < 2 {
            return;
        }
        let prev_index = self.current;
        let next_index = (self.current + 1) % self.task_count;
        self.current = next_index;

        let prev_task = self.tasks[prev_index].as_mut().unwrap();
        prev_task.state = TaskState::Ready;

        // assing a raw pointer:
        let prev_sp_ptr = &raw mut prev_task.sp;
        let next_sp = self.tasks[next_index].as_ref().unwrap().sp;
        self.tasks[next_index].as_mut().unwrap().state = TaskState::Running;

        unsafe {
            switch_context(prev_sp_ptr, &next_sp);
        }
    }
}

pub static mut SCHEDULER: Scheduler = Scheduler::new();

pub fn task_yield() {
    unsafe {
        let sched = &raw mut SCHEDULER;
        (*sched).schedule_next();
    }
}
