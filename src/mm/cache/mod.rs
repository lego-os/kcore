mod alloc;
mod kc;
mod slub_entry;

use core::{alloc::Layout, ptr::NonNull};

use crate::mm::Allocator;

use super::{AllocError, GfpFlags, Order};
use alloc::SlubAllocator;

pub const MIN_OBJECT_SIZE: usize = 8;
pub const MAX_OBJECT_SIZE: usize = 2048;
pub const FIX_SLUB_NUM: usize =
    (MAX_OBJECT_SIZE.trailing_zeros() - MIN_OBJECT_SIZE.trailing_zeros() + 1) as _;
pub const SLUB_ORDER: Order = Order::Ten;

static SLUB_ALLOCATOR: SlubAllocator = SlubAllocator::new();

pub struct KMemAlloc;

unsafe impl Allocator for KMemAlloc {
    fn alloc(layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        todo!()
    }

    unsafe fn free(ptr: NonNull<u8>, layout: Layout) {
        todo!()
    }
}

pub fn init_slub_allocator() {
    SLUB_ALLOCATOR.init();
}

#[inline]
pub fn alloc(layout: Layout, flags: GfpFlags) -> Result<NonNull<[u8]>, AllocError> {
    SLUB_ALLOCATOR.alloc(layout, flags)
}

pub fn free(ptr: NonNull<u8>) {
    unsafe { SLUB_ALLOCATOR.free(ptr) }
}
