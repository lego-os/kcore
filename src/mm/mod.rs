mod cache;
mod gfp_falg;
mod page_map;
mod page_alloc;
mod vmalloc;
mod mem;

pub(crate) use gfp_falg::*;
pub(crate) use page_map::*;

pub use cache::KMemAlloc;
pub use cache::{alloc, free, init_slub_allocator};
pub use page_alloc::*;
pub use vmalloc::KVMAlloc;

use core::{
    alloc::Layout,
    error::Error,
    fmt::{Display, Formatter},
    ptr::{self, NonNull},
};

pub const PAGE_SIZE: usize = 4096;

/// Indicates an allocation error.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AllocError;

impl Error for AllocError {}

impl Display for AllocError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("memory allocation failed")
    }
}

pub unsafe trait Allocator {
    fn alloc(layout: Layout) -> Result<NonNull<[u8]>, AllocError>;

    fn alloc_zeroed(layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = Self::alloc(layout)?;
        unsafe { (ptr.as_ptr() as *mut u8).write_bytes(0, ptr.len()) }
        Ok(ptr)
    }

    unsafe fn free(ptr: NonNull<u8>, layout: Layout);

    unsafe fn grow(
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        debug_assert!(
            new_layout.size() >= old_layout.size(),
            "`new_layout.size()` must be greater than or equal to `old_layout.size()`"
        );

        let new_ptr = Self::alloc(new_layout)?;
        unsafe {
            ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_ptr() as *mut u8, old_layout.size());
            Self::free(ptr, old_layout);
        }

        Ok(new_ptr)
    }

    unsafe fn grow_zeroed(
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        debug_assert!(
            new_layout.size() >= old_layout.size(),
            "`new_layout.size()` must be greater than or equal to `old_layout.size()`"
        );

        let new_ptr = Self::alloc_zeroed(new_layout)?;

        // SAFETY: because `new_layout.size()` must be greater than or equal to
        // `old_layout.size()`, both the old and new memory allocation are valid for reads and
        // writes for `old_layout.size()` bytes. Also, because the old allocation wasn't yet
        // deallocated, it cannot overlap `new_ptr`. Thus, the call to `copy_nonoverlapping` is
        // safe. The safety contract for `dealloc` must be upheld by the caller.
        unsafe {
            ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_ptr() as *mut u8, old_layout.size());
            Self::free(ptr, old_layout);
        }

        Ok(new_ptr)
    }

    unsafe fn shrink(
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        debug_assert!(
            new_layout.size() <= old_layout.size(),
            "`new_layout.size()` must be smaller than or equal to `old_layout.size()`"
        );

        let new_ptr = Self::alloc(new_layout)?;
        unsafe {
            ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_ptr() as *mut u8, new_layout.size());
            Self::free(ptr, old_layout);
        }

        Ok(new_ptr)
    }
}
