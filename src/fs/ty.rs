use core::fmt::Display;

/// 文件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    RegularFile = 1,
    Directory = 1 << 1,
    SoftLink = 1 << 2,
    HardLink = 1 << 3,
    BlockDevice = 1 << 4,
    CharDevice = 1 << 5,
    NetSocket = 1 << 6,
    Pipe = 1 << 7,
}

impl Display for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Type::RegularFile => write!(f, "r"),
            Type::Directory => write!(f, "d"),
            Type::SoftLink => write!(f, "s"),
            Type::HardLink => write!(f, "h"),
            Type::BlockDevice => write!(f, "b"),
            Type::CharDevice => write!(f, "c"),
            Type::NetSocket => write!(f, "n"),
            Type::Pipe => write!(f, "p"),
        }
    }
}
