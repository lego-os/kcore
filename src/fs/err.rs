use core::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsErr {
    PermissionDefined,
    NotFound,
    TooManyFiles,
    IOErr,
    ResourceBusy,
    InvalidArgument,
}

impl Error for FsErr {}

impl Display for FsErr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}
