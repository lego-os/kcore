use super::{SLUB_ORDER, slub_entry::Slub};
use crate::{
    link_ptr,
    mm::{GfpFlags, PG_SLUB, PageFlags, alloc_pages},
    node_ptr,
    utils::mem_ds::SLink,
};
use core::{ptr::NonNull, slice};

pub struct KMemCache {
    slub_head: Option<NonNull<Slub>>,
    slub_num: usize,
    obj_size: u32,
}

impl KMemCache {
    pub const fn new() -> Self {
        Self {
            slub_head: None,
            slub_num: 0,
            obj_size: 0,
        }
    }

    pub fn init(&mut self, obj_size: u32) {
        self.obj_size = obj_size;
    }

    pub fn alloc_obj(&mut self) -> NonNull<[u8]> {
        if self.slub_num == 0 {
            self.slub_head = Some(self.alloc_slub());
        }
        let mut slub = unsafe { self.slub_head.unwrap().as_mut() };
        loop {
            if slub.empty() {
                match slub.next {
                    Some(s) => {
                        let node = node_ptr!(Slub, s.as_ptr(), next);
                        slub = unsafe { node.as_mut().unwrap() };
                    }
                    None => {
                        let mut new_slub = self.alloc_slub();
                        let head_ptr = self.slub_head.unwrap().as_ptr();
                        let head_link = link_ptr!(SLink, Slub, head_ptr, next);
                        unsafe {
                            (*new_slub.as_ptr()).next = Some(NonNull::new_unchecked(head_link));
                        }
                        self.slub_head = Some(new_slub);
                        slub = unsafe { new_slub.as_mut() }
                    }
                }
            } else {
                let obj = slub.get_free_object();
                return unsafe {
                    NonNull::new_unchecked(slice::from_raw_parts_mut(
                        obj.as_ptr(),
                        self.obj_size as usize,
                    ))
                };
            }
        }
    }

    #[inline]
    fn alloc_slub(&self) -> NonNull<Slub> {
        let psr = alloc_pages(SLUB_ORDER, GfpFlags::default()).unwrap();
        psr.inner.lock().flags = PageFlags(PG_SLUB);
        Slub::new_from_psr(psr, self.obj_size)
    }
}
