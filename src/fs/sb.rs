use crate::fs::DEntry;


pub struct SuperBlock{
    magic:u64,
    dev_id:u32,
    blk_size:u32,
    blk_bits:u8,
    mfs:u64,
    root:DEntry,
    nr_inode:u64,
    nr_log:u64,
    nr_data:u64,
    
}