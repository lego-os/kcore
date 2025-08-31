use core::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevErr {
    NoSuch,
    IOErr,
    ResourceBusy,
    
}

impl Error for DevErr {}

impl Display for DevErr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}
