use core::{ptr::NonNull, sync::atomic::AtomicPtr};

use crate::fs::File;

const OPEN_FILE_NR: usize = 32;

struct Context {
    ra: u64,
    sp: u64,
    s0: u64,
    s1: u64,
    s2: u64,
    s3: u64,
    s4: u64,
    s5: u64,
    s6: u64,
    s7: u64,
    s8: u64,
    s9: u64,
    s10: u64,
    s11: u64,
}

struct Core {
    task: AtomicPtr<Task>,
    context: Context,
    intr_able: i32,
}

pub struct Task {
    name: [u8; 16],
    killed: i32,
    xstate: i32,
    state:TaskState,
    kstack: u64,
    ofs: [NonNull<File>; OPEN_FILE_NR],
    parent: NonNull<Task>,
    pte: u64,
    tid:u32,
    size:u64,
    trap_frame:TrapFrame,
    context:Context,
    // cur_dir:NonNull<Inode>,
}

#[derive(Debug,Clone, Copy,PartialEq, Eq)]
pub enum TaskState {
    Unused,
    Used,
    Sleeping,
    Runnable,
    Running,
    Zombie,
}
struct TrapFrame {
    ksatp: u64,
    ksp: u64,
    ktrap: u64,
    epc: u64,
    kcore_id: u64,
    ra: u64,
    sp: u64,
    gp: u64,
    tp: u64,
    t0: u64,
    t1: u64,
    t2: u64,
    s0: u64,
    s1: u64,
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
    s2: u64,
    s3: u64,
    s4: u64,
    s5: u64,
    s6: u64,
    s7: u64,
    s8: u64,
    s9: u64,
    s10: u64,
    s11: u64,
    t3: u64,
    t4: u64,
    t5: u64,
    t6: u64,
}
