mod alloc;
mod list;
mod order;
mod per_core;
pub use order::*;

use crate::utils::sync::SpinLock;
use alloc::BuddyAllocator;

static BUDDY_ALLOCATOR: SpinLock<Option<BuddyAllocator>> = SpinLock::new(None);

// pub fn init_buddy_allocator(mem_start: usize, total_size: usize, start_addr: usize) {
//     let mut guard = BUDDY_ALLOCATOR.lock();
//     if guard.is_none() {
//         let mut buddy = BuddyAllocator::new();
//         buddy.init(mem_start, total_size, start_addr);
//         guard.replace(buddy);
//     } else {
//         panic!("Buddy Allocator has initialized!")
//     }
// }




pub(super) 