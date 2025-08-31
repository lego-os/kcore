use core::ptr::NonNull;

use crate::{
    fs::{Inode, SuperBlock},
    utils::{arc::VArc, string::VString, sync::RWLockSp},
};

pub struct DFlags(u32);

pub enum DFlag {}

pub struct DEntry {
    flags: DFlags,
    parent: Option<VArc<RWLockSp<Self>>>,
    name: VString,
    sb: VArc<SuperBlock>,
}

pub trait DEntryOps {}
