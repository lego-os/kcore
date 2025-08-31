use super::{
    FIX_SLUB_NUM, MAX_OBJECT_SIZE, MIN_OBJECT_SIZE, SLUB_ORDER, cache::KMemCache, slub_entry::Slub,
};
use crate::{
    mm::{AllocError, GfpFlags, PageSliceRef},
    utils::sync::SpinLock,
};
use core::{alloc::Layout, ptr::NonNull};

pub struct SlubAllocator([SpinLock<KMemCache>; FIX_SLUB_NUM]);

impl SlubAllocator {
    pub const fn new() -> Self {
        Self([const { SpinLock::new(KMemCache::new()) }; FIX_SLUB_NUM])
    }

    pub fn init(&self) {
        self.0.iter().for_each(|cache| {
            let mut guard = cache.lock();
            guard.init(1);
        });
    }

    pub fn alloc(&self, layout: Layout, _flags: GfpFlags) -> Result<NonNull<[u8]>, AllocError> {
        let size = layout.size();
        let index = if size < MIN_OBJECT_SIZE {
            0
        } else if size > MAX_OBJECT_SIZE {
            panic!("alloc memory too big!");
        } else {
            let valid_bits = usize::BITS - size.leading_zeros() - 1;
            if 1 << valid_bits == size as u32 {
                valid_bits
            } else {
                valid_bits + 1
            }
        };
        let mut guard = self.0[index as usize].lock();
        Ok(guard.alloc_obj())
    }

    pub unsafe fn free(&self, ptr: NonNull<u8>) {
        let psr_addr = SLUB_ORDER.as_size() as usize & ptr.addr().get();
        let psr = PageSliceRef::from_raw(psr_addr as *const usize);
        let slub = Slub::from_psr(psr);
        unsafe {
            (*slub.as_ptr()).push_object(ptr);
        }
    }
}

unsafe impl Send for SlubAllocator {}
unsafe impl Sync for SlubAllocator {}
