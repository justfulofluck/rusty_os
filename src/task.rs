#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
}

/// RISC-V 32-bit architecture registers Frame.
#[allow(dead_code)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ContextFrame {
    // return address
    pub mepc: usize,
    pub ra: usize,

    // temporary registers
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,

    // saved registers
    pub s0: usize,
    pub s1: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,

    // global registers or pointer & thread pointers
    pub gp: usize,
    pub tp: usize,
}

impl ContextFrame {
    // for new task
    pub fn new(entry_point: usize) -> Self {
        Self {
            mepc: entry_point,
            ra: 0,
            t0: 0,
            t1: 0,
            t2: 0,
            t3: 0,
            t4: 0,
            t5: 0,
            t6: 0,
            s0: 0,
            s1: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            s8: 0,
            s9: 0,
            s10: 0,
            s11: 0,
            gp: 0,
            tp: 0,
        }
    }
}

#[allow(dead_code)]
/// task control block
pub struct TaskControlBlock {
    pub id: usize,
    pub sp: usize,
    pub state: TaskState,
    pub priority: u8,
}

impl TaskControlBlock {
    pub fn new(id: usize, priority: u8, stack_top: usize, entry_point: fn() -> !) -> Self {
        let aligned_stack_top = stack_top & !0xF;
        let context = core::mem::size_of::<ContextFrame>();
        let frame_ptr = (aligned_stack_top - context) as *mut ContextFrame;

        unsafe {
            core::ptr::write(frame_ptr, ContextFrame::new(entry_point as usize));
        }

        Self {
            id,
            sp: frame_ptr as usize,
            state: TaskState::Ready,
            priority,
        }
    }
}

#[naked]
#[no_mangle]
pub unsafe extern "C" fn switch_context(current_sp: *mut usize, next_sp: *const usize) {
    naked_asm!(
        // staack allotment provide 128 bits
        "addi sp, sp, -128",

        //to save return and register address
        "sw ra, 0(sp)"
        "sw t0, 4(sp)",
        "sw t1, 8(sp)",
        "sw t2, 12(sp)",
        "sw s0, 16(sp)",
        "sw s1, 20(sp)",
        "sw a0, 24(sp)",
        "sw a1, 28(sp)",
        "sw a2, 32(sp)",
        "sw a3, 36(sp)",
        "sw a4, 40(sp)",
        "sw a5, 44(sp)",
        "sw a6, 48(sp)",
        "sw a7, 52(sp)",
        "sw s2, 56(sp)",
        "sw s3, 60(sp)",
        "sw s4, 64(sp)",
        "sw s5, 68(sp)",
        "sw s6, 72(sp)",
        "sw s7, 76(sp)",
        "sw s8, 80(sp)",
        "sw s9, 84(sp)",
        "sw s10, 88(sp)",
        "sw s11, 92(sp)",
        "sw t3, 96(sp)",
        "sw t4, 100(sp)",
        "sw t5, 104(sp)",
        "sw t6, 108(sp)",

        // 3. to save current stack pointer in *current_sp (a0)
        "sw sp, 0(a0)",

        // 4. Next Task ka Stack Pointer load karein next_sp (a1) se
        "lw sp, 0(a1)",

        // 5. to register a new task
        "lw ra, 0(sp)",
        "lw t0, 4(sp)",
        "lw t1, 8(sp)",
        "lw t2, 12(sp)",
        "lw s0, 16(sp)",
        "lw s1, 20(sp)",
        "lw a0, 24(sp)",
        "lw a1, 28(sp)",
        "lw a2, 32(sp)",
        "lw a3, 36(sp)",
        "lw a4, 40(sp)",
        "lw a5, 44(sp)",
        "lw a6, 48(sp)",
        "lw a7, 52(sp)",
        "lw s2, 56(sp)",
        "lw s3, 60(sp)",
        "lw s4, 64(sp)",
        "lw s5, 68(sp)",
        "lw s6, 72(sp)",
        "lw s7, 76(sp)",
        "lw s8, 80(sp)",
        "lw s9, 84(sp)",
        "lw s10, 88(sp)",
        "lw s11, 92(sp)",
        "lw t3, 96(sp)",
        "lw t4, 100(sp)",
        "lw t5, 104(sp)",
        "lw t6, 108(sp)",

        // 6. Stack pointer deallocate karein
        "addi sp, sp, 128",

        // 7. Naye task ke ra (Return Address) par jump karein
        "ret"
    )
}
