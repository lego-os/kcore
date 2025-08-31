use crate::{mm::PageSliceRef, utils::mem_ds::SLink};
use core::{mem, ptr::NonNull};
pub struct Slub {
    obj_num: u32,
    free_num: u32,
    obj_size: u32,
    size: u32,
    free_head: Option<NonNull<SLink>>,
    pub(super) next: Option<NonNull<SLink>>,
}

impl Slub {
    pub fn new_from_psr(psr: PageSliceRef, obj_size: u32) -> NonNull<Self> {
        let size = psr.inner.lock().size;
        let limit = psr.available_ptr().addr().get() + size_of::<Self>();
        let mut obj_num = 1;
        let mut obj: NonNull<SLink> =
            unsafe { mem::transmute(psr.as_ptr().add((size - obj_size) as _)) };
        loop {
            let new_obj = unsafe { obj.sub(obj_size as _) };
            if new_obj.addr().get() < limit {
                break;
            }
            unsafe {
                (*new_obj.as_ptr()).next = Some(obj);
            }
            obj = new_obj;
            obj_num += 1;
        }
        let mut slub: NonNull<Slub> = unsafe { mem::transmute(psr.available_ptr()) };
        let slub_mut = unsafe { slub.as_mut() };
        slub_mut.free_head = Some(obj);
        slub_mut.size = size;
        slub_mut.free_num = obj_num;
        slub_mut.obj_num = obj_num;
        slub_mut.next = None;
        slub_mut.obj_size = obj_size;
        slub
    }

    #[inline]
    pub fn from_psr(psr: PageSliceRef) -> NonNull<Self> {
        unsafe { mem::transmute(psr.available_ptr()) }
    }

    pub fn get_free_object(&mut self) -> NonNull<u8> {
        let head = self.free_head.unwrap();
        let next = unsafe { (*head.as_ptr()).next };
        self.free_head = next;
        unsafe { (*head.as_ptr()).next = None };
        self.free_num -= 1;
        unsafe { mem::transmute(head) }
    }

    pub fn push_object(&mut self, obj: NonNull<u8>) {
        let obj: NonNull<SLink> = unsafe { mem::transmute(obj) };
        unsafe { (*obj.as_ptr()).next = self.free_head }
        self.free_head = Some(obj);
        self.free_num += 1;
    }

    pub fn empty(&self) -> bool {
        self.free_num == 0
    }
}
