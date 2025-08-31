pub const VM_USER_OFFSET: usize = 0x0000000000000000;
pub const VM_USER_SIZE: usize = 0x0000000000000000;

pub const VM_FIXMAP_OFFSET: usize = 0x0000000000000000;
pub const VM_FIXMAP_SIZE: usize = 0x0000000000000000;

pub const VM_MEMMAP_OFFSET: usize = 0x0000000000000000;
pub const VM_MEMMAP_SIZE: usize = 0x0000000000000000;

pub const VM_VMALLOC_OFFSET: usize = 0xffffffc500000000;
pub const VM_VMALLOC_SIZE: usize = 0x0000004000000000;

pub const VM_ALLPHY_OFFSET: usize = 0xffffffc500000000;
pub const VM_ALLPHY_SIZE: usize = 0x0000004000000000;

pub const VM_MODULES_OFFSET: usize = 0xffffffc500000000;
pub const VM_MODULES_SIZE: usize = 0x0000004000000000;

pub const VM_KERNEL_OFFSET: usize = 0xffffffc500000000;
pub const VM_KERNEL_SIZE: usize = 0x0000004000000000;

pub const PAGE_SIZE: usize = 4096;
pub const PAGE_POWER:u8 = PAGE_SIZE.trailing_zeros() as _;

unsafe extern "C" {
    fn kernel_end();
}

#[macro_export]
macro_rules! KEND {
    () => {
        unsafe { kernel_end as usize }
    };
}
