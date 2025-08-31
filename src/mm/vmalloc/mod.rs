use crate::mm::Allocator;
mod rbtree;

pub struct KVMAlloc;

unsafe impl Allocator for KVMAlloc {
    fn alloc(layout: core::alloc::Layout) -> Result<core::ptr::NonNull<[u8]>, super::AllocError> {
        todo!()
    }

    unsafe fn free(ptr: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        todo!()
    }
}
