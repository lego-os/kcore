use core::alloc::Layout;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Page {
    pub layout: Layout,
    // pub flags: ApFlags,
    pub addr: usize,
    pub access: usize,
}