use core::ptr::NonNull;

pub struct SLink {
    pub next: Option<NonNull<SLink>>,
}

pub struct DLink {
    pub next: Option<NonNull<DLink>>,
    pub prev: Option<NonNull<DLink>>,
}
