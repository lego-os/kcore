use crate::utils::sync::OnceLock;
use alloc::boxed::Box;

pub(crate) struct Memory {
    slaves:OnceLock<Box<[MemSection]>>,
    
}

pub(crate) struct MemSection {
    start: u64,
    size: u64,
    id: u32,
}
