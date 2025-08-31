
pub struct File{
    
}

use crate::fs::FsErr;

pub trait Read {
    fn read(&mut self,buf:&mut [u8])->Result<usize,FsErr>;
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize,FsErr>;
}

pub trait Seek {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64,FsErr>;
}

pub enum SeekFrom{
    Start(u64),
    End(i64),
    Current(i64),
}

pub struct FModes(u8);

pub enum FMode{
    
}