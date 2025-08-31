use core::{
    fmt::Display,
    ptr::{self, NonNull},
    sync::atomic::{AtomicU8, AtomicU32, AtomicUsize, Ordering},
};

pub const KPAGE_ID: isize = isize::MIN;

pub struct VMemMap {
    map_start: *const PageInner,
    len: usize,
}

impl VMemMap {
    pub(super) const fn new(start: *const u8) -> Self {
        Self {
            map_start: start as *const _,
            len: 0,
        }
    }
}

pub struct PageSlice {
    virt_addr: NonNull<u8>,
    order: u8,
}

impl PageSlice {
    fn from_page_inner(pi: &PageInner) -> Self {
        Self {
            virt_addr: unsafe { NonNull::new_unchecked(pi.virt_addr as *mut _) },
            order: pi.order.load(Ordering::Acquire),
        }
    }
}

#[repr(align(4))]
struct PageInner {
    virt_addr: *const u8,
    ref_count: AtomicUsize,
    flag: AtomicU32,
    order: AtomicU8,
}

impl PageInner {
    const fn new() -> Self {
        Self {
            ref_count: AtomicUsize::new(0),
            order: AtomicU8::new(0),
            flag: AtomicU32::new(PageFlag::Kernel as _),
            virt_addr: ptr::null_mut(),
        }
    }
}

unsafe impl Sync for PageInner {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageFlag {
    Kernel = 0,
    BuddyNormal = 1,
    BuddyCma = 1 << 1,
    BuddyEmg = 1 << 2,
    SlubNormal = 1 << 3,
    SlubEmg = 1 << 4,
}

impl Display for PageFlag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PageFlag::BuddyNormal => write!(f, "BuddyNormal"),
            PageFlag::BuddyCma => write!(f, "BuddyCma"),
            PageFlag::BuddyEmg => write!(f, "BuddyEmg"),
            PageFlag::SlubNormal => write!(f, "SlubNormal"),
            PageFlag::SlubEmg => write!(f, "SlubEmg"),
            PageFlag::Kernel => write!(f, "Kernel"),
        }
    }
}
