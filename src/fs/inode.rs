// int (*create) (struct mnt_idmap *, struct inode *,struct dentry *, umode_t, bool);
//         struct dentry * (*lookup) (struct inode *,struct dentry *, unsigned int);
//         int (*link) (struct dentry *,struct inode *,struct dentry *);
//         int (*unlink) (struct inode *,struct dentry *);
//         int (*symlink) (struct mnt_idmap *, struct inode *,struct dentry *,const char *);
//         struct dentry *(*mkdir) (struct mnt_idmap *, struct inode *,struct dentry *,umode_t);
//         int (*rmdir) (struct inode *,struct dentry *);
//         int (*mknod) (struct mnt_idmap *, struct inode *,struct dentry *,umode_t,dev_t);
//         int (*rename) (struct mnt_idmap *, struct inode *, struct dentry *,
//                        struct inode *, struct dentry *, unsigned int);
//         int (*readlink) (struct dentry *, char __user *,int);
//         const char *(*get_link) (struct dentry *, struct inode *,
//                                  struct delayed_call *);
//         int (*permission) (struct mnt_idmap *, struct inode *, int);
//         struct posix_acl * (*get_inode_acl)(struct inode *, int, bool);
//         int (*setattr) (struct mnt_idmap *, struct dentry *, struct iattr *);
//         int (*getattr) (struct mnt_idmap *, const struct path *, struct kstat *, u32, unsigned int);
//         ssize_t (*listxattr) (struct dentry *, char *, size_t);
//         void (*update_time)(struct inode *, struct timespec *, int);
//         int (*atomic_open)(struct inode *, struct dentry *, struct file *,
//                            unsigned open_flag, umode_t create_mode);
//         int (*tmpfile) (struct mnt_idmap *, struct inode *, struct file *, umode_t);
//         struct posix_acl * (*get_acl)(struct mnt_idmap *, struct dentry *, int);
//         int (*set_acl)(struct mnt_idmap *, struct dentry *, struct posix_acl *, int);
//         int (*fileattr_set)(struct mnt_idmap *idmap,
//                             struct dentry *dentry, struct fileattr *fa);
//         int (*fileattr_get)(struct dentry *dentry, struct fileattr *fa);
//         struct offset_ctx *(*get_offset_ctx)(struct inode *inode);


use crate::{
    fs::{Pms, SuperBlock, Type},
    utils::{arc::{KArc, VArc}},
};

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct IOpsFlags(pub u32);

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct IFlags(pub u32);

pub enum InodeState {}

pub struct Inode {
    pub mode:InodeMode,
   //  pub sb: KArc<RWLockSp<>>,
    pub size: u64,
    pub crt_tsp: u64,
    pub acs_tsp: u64,
    pub mcs_tsp: u64,
    pub ino: u64,
    pub link: u32,
    pub dev_id: u32,
    pub version: u32,
    pub blk_order: u8,
    pub blk_nr: u32,
    pub state: InodeState,
}

impl Inode {
    
}

/// 
#[derive(Debug,Copy,Clone)]
pub struct InodeMode(u16);

pub enum InodeType{
    Dir = 1,
    File = 1 << 1,
    Link = 1 << 2,
}

pub trait InodeOps {
    fn create()->Result<Inode>;
    fn lookup();
    fn link();
    fn unlink();
    fn symlink();
    fn mkdir();
    fn rmdir();
    fn mknod();
    fn rename();
    fn readlink();
    fn get_link();
    fn pms();
    fn set_attr();
    fn get_attr();
    fn list_xattr();
    fn update_time();
    fn atomic_open();
    fn tmpfile();
}
