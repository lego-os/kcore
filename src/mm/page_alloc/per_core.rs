use crate::mm::{page_alloc::list::BlockList, MAX_ORDER};
pub(crate) struct PerCoreBuddyAlloc([BlockList;MAX_ORDER]);



