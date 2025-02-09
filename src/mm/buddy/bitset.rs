#[derive(Debug, Default, Clone, Copy)]
pub(super) struct MemBitSet {
    start_addr: usize,
    quantity: usize,
    size: usize,
}

impl MemBitSet {
    #[inline]
    pub const fn new() -> Self {
        Self {
            start_addr: 0,
            quantity: 0,
            size: 0,
        }
    }

    #[inline]
    pub fn init(&mut self, start_addr: usize, quantity: usize) {
        self.quantity = quantity;
        self.start_addr = start_addr;
        let mut size = (quantity / u8::BITS as usize) * size_of::<u8>();
        if quantity % u8::BITS as usize != 0 {
            size += size_of::<u8>();
        }
        unsafe {
            (start_addr as *mut u8).write_bytes(u8::MAX, size);
        }
        self.size = size;
    }

    pub fn is_set(&self, index: usize) -> bool {
        if index >= self.quantity {
            return true;
        }
        let mask = (1 << (index % u8::BITS as usize)) as u8;
        let addr = index / u8::BITS as usize + self.start_addr;
        let ptr = addr as *mut u8;
        unsafe { (*ptr) & mask != 0 }
    }

    pub fn set_bit(&self, index: usize) {
        assert!(index < self.quantity);
        let mask = (1 << (index % u8::BITS as usize)) as u8;
        let addr = index / u8::BITS as usize + self.start_addr;
        let ptr = addr as *mut u8;
        unsafe {
            assert!(*ptr & mask == 0);
            ptr.write((*ptr) | mask);
        }
    }

    pub fn clear_bit(&self, index: usize) {
        assert!(index < self.quantity);
        let mask = (1 << (index % u8::BITS as usize)) as u8;
        let addr = index / u8::BITS as usize + self.start_addr;
        let ptr = addr as *mut u8;
        unsafe {
            assert!(*ptr & mask != 0);
            ptr.write(*ptr & !mask);
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }
}
