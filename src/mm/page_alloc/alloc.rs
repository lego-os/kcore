use crate::{
    arch::mm::PAGE_POWER,
    mm::{AllocError, GfpFlags, PageSlice},
};
use core::{
    cmp,
    ptr::{self, NonNull},
};

use super::{MAX_ORDER, Order, list::BlockList};

pub struct BuddyAllocator {
    free_lists: [BlockList; MAX_ORDER + 1],
    start_addr: usize,
    total_size: usize,
    available_size: usize,
    min_order: u8,
    max_order: u8,
}

impl BuddyAllocator {
    pub const fn new(min_order: u8, max_order: u8) -> Self {
        Self {
            free_lists: [const { BlockList::new() }; MAX_ORDER + 1],
            start_addr: 0,
            total_size: 0,
            available_size: 0,
            min_order,
            max_order,
        }
    }

    pub fn init(&mut self, start_addr: usize, size: usize) {
        let min_blk: usize = 1 << (self.min_order + PAGE_POWER);
        if start_addr % min_blk != 0 || size % min_blk != 0 {
            panic!("The start_addr or total_size addresses are misaligned.");
        }

        let mut order = self.max_order;
        let mut addr = start_addr + size;
        while addr >= start_addr {
            let next_addr = addr - 1 << (PAGE_POWER + order);
            addr = if next_addr < start_addr {
                order -= 1;
                continue;
            } else {
                next_addr
            };
            self.free_lists[order as usize]
                .push(unsafe { NonNull::new_unchecked(addr as *mut u8) });
        }
        self.start_addr = start_addr;
        self.available_size = size;
        self.total_size = size;
    }

    pub fn alloc_pages(
        &mut self,
        order: Order,
        _flags: GfpFlags,
    ) -> Result<PageSlice, AllocError> {
        let mut target_order = order.as_power();
        loop {
            if target_order > self.max_order {
                return Err(AllocError);
            }
            if !self.free_lists[target_order as usize].empty() {
                break;
            }
            target_order += 1;
        }
        let psr = PageSliceRef::from_end(self.free_lists[target_order].pop().unwrap().as_ptr());
        let ps_addr = psr.ps_addr();
        for ord in (order.as_power()..target_order).rev() {
            let ps_size = Order::from_order(ord).as_size();
            let new_psr = PageSliceRef::from_raw((ps_addr + ps_size as usize) as *const usize);
            {
                let mut inner = new_psr.inner.lock();
                inner.flags = PageFlags(PG_BUDDY);
                inner.size = ps_size;
                inner.virt_addr = ptr::null();
            }
            self.free_lists[ord].push(new_psr.available_ptr());
        }
        {
            let mut inner = psr.inner.lock();
            inner.size = order.as_size();
            inner.flags = PageFlags(PG_INUSE);
            inner.virt_addr = ptr::null();
        }
        self.available_size -= order.as_size() as usize;
        Ok(psr)
    }

    pub fn free_pages(&mut self, psr: PageSliceRef, order: Order) -> Result<(), AllocError> {
        let mut ps = psr.inner.lock();
        if !ps.flags.contains(PageFlags(PG_INUSE)) {
            return Err(AllocError);
        }
        ps.flags = PageFlags(PG_IDLE);
        let mut ps_addr = psr.ps_addr();
        for ord in order.as_power()..MAX_ORDER {
            let ps_size = Order::from_order(ord).as_size();
            let buddy_ps = PageSliceRef::from_raw((ps_addr ^ (ps_size as usize)) as *const usize);
            let (size, flags) = {
                let inner = buddy_ps.inner.lock();
                (inner.size, inner.flags)
            };
            if flags.contains(PageFlags(PG_BUDDY)) && size == Order::from_order(ord).as_size() {
                ps_addr = cmp::min(ps_addr, buddy_ps.ps_addr());
                buddy_ps.inner.lock().flags = PageFlags(PG_IDLE);
                self.free_lists[ord].remove(buddy_ps.available_ptr());
            } else {
                let psr = PageSliceRef::from_raw(ps_addr as *const usize);
                {
                    let mut inner = psr.inner.lock();
                    inner.flags = PageFlags(PG_BUDDY);
                    inner.size = ps_size;
                    inner.virt_addr = ptr::null();
                }
                self.free_lists[ord].push(psr.available_ptr());
            }
        }
        self.available_size += order.as_size() as usize;
        Ok(())
    }
}

unsafe impl Send for BuddyAllocator {}
unsafe impl Sync for BuddyAllocator {}
