use core::{mem, ptr::NonNull};

pub(super) struct BlockList {
    head: Option<NonNull<BlockNode>>,
    tail: Option<NonNull<BlockNode>>,
    blk_num: usize,
}

struct BlockNode {
    next: Option<NonNull<Self>>,
    prev: Option<NonNull<Self>>,
}

impl BlockList {
    pub const fn new() -> Self {
        Self {
            head: None,
            tail: None,
            blk_num: 0,
        }
    }

    /// 对入队元素是否为空不做任何判断
    pub fn push(&mut self, blk: NonNull<u8>) {
        let blk = unsafe { mem::transmute::<NonNull<u8>, NonNull<BlockNode>>(blk) };
        let blk_mut = unsafe { &mut *blk.as_ptr() };
        blk_mut.prev = self.tail;
        blk_mut.next = None;
        if self.blk_num == 0 {
            self.head = Some(blk);
        }
        self.tail = Some(blk);
        self.blk_num += 1;
    }

    pub fn pop(&mut self) -> Option<NonNull<u8>> {
        if self.blk_num == 0 {
            return None;
        }
        let head_blk = self.head.as_mut().unwrap().as_ptr();
        let next = unsafe { (*head_blk).next };
        if next.is_some() {
            unsafe { (*next.as_ref().unwrap().as_ptr()).prev = None }
        }
        self.head = next;
        self.blk_num -= 1;
        if self.blk_num == 0 {
            self.tail = None;
        }
        Some(unsafe { NonNull::new_unchecked(head_blk as _) })
    }

    pub const fn remove(&mut self, blk: NonNull<u8>) {
        if self.blk_num == 0 {
            panic!("[Buddy Allocator]: BlockQueue::remove -> The queue is empty.");
        }
        let blk = unsafe { &mut *mem::transmute::<NonNull<u8>, NonNull<BlockNode>>(blk).as_ptr() };
        let (prev, next) = (blk.prev, blk.next);
        blk.next = None;
        blk.prev = None;

        match prev {
            Some(ptr) => unsafe {
                (*ptr.as_ptr()).next = next;
            },
            None => self.head = next,
        }
        match next {
            Some(ptr) => unsafe { (*ptr.as_ptr()).prev = prev },
            None => self.tail = prev,
        }
        self.blk_num -= 1;
    }
}
