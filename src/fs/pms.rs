use core::{
    fmt::Display,
    ops::{BitAnd, BitOr},
};

/// 无任何权限 NONE(0)
pub const PERMISSION_NONE: Pms = Pms(0);
/// 仅执行 X(0b1)
pub const PERMISSION_EO: Pms = Pms(PmsBit::Execute as _);
/// 仅写 W(0b10)
pub const PERMISSION_WO: Pms = Pms(PmsBit::Write as _);
/// 仅读 R(0b100)
pub const PERMISSION_RO: Pms = Pms(PmsBit::Read as _);
/// 读写 RW(0b110)
pub const PERMISSION_RW: Pms =
    Pms(PmsBit::Read as u8 | PmsBit::Write as u8);
/// 读执行 RX(0b101)
pub const PERMISSION_RX: Pms =
    Pms(PmsBit::Read as u8 | PmsBit::Execute as u8);
/// 写执行 WX(0b11)
pub const PERMISSION_WX: Pms =
    Pms(PmsBit::Execute as u8 | PmsBit::Write as u8);
/// 读写执行 RWX(0b111)
pub const PERMISSION_RWX: Pms =
    Pms(PmsBit::Read as u8 | PmsBit::Write as u8 | PmsBit::Execute as u8);


/// 文件权限
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Pms(u8);

impl Pms {
    #[inline]
    pub fn contains(&self, permission: PmsBit) -> bool {
        permission & self.0 != 0
    }
}

impl Display for Pms {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.contains(PmsBit::Read) {
            write!(f, "{}", PmsBit::Read)?;
        } else {
            write!(f, "{}", PmsBit::None)?;
        }
        if self.contains(PmsBit::Write) {
            write!(f, "{}", PmsBit::Write)?;
        } else {
            write!(f, "{}", PmsBit::None)?;
        }
        if self.contains(PmsBit::Execute) {
            write!(f, "{}", PmsBit::Execute)?;
        } else {
            write!(f, "{}", PmsBit::None)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PmsBit {
    None = 0,
    Execute = 1,
    Write = 1 << 1,
    Read = 1 << 2,
}

impl Display for PmsBit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PmsBit::None => write!(f, "-"),
            PmsBit::Execute => write!(f, "x"),
            PmsBit::Write => write!(f, "w"),
            PmsBit::Read => write!(f, "r"),
        }
    }
}

impl BitAnd for PmsBit {
    type Output = u8;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        self as u8 & rhs as u8
    }
}

impl BitAnd<u8> for PmsBit {
    type Output = u8;
    #[inline]
    fn bitand(self, rhs: u8) -> Self::Output {
        self as u8 & rhs
    }
}

impl BitAnd<PmsBit> for u8 {
    type Output = u8;
    #[inline]
    fn bitand(self, rhs: PmsBit) -> Self::Output {
        self & rhs as u8
    }
}

impl BitOr for PmsBit {
    type Output = u8;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        self as u8 | rhs as u8
    }
}

impl BitOr<u8> for PmsBit {
    type Output = u8;
    #[inline]
    fn bitor(self, rhs: u8) -> Self::Output {
        self as u8 | rhs
    }
}

impl BitOr<PmsBit> for u8 {
    type Output = u8;
    #[inline]
    fn bitor(self, rhs: PmsBit) -> Self::Output {
        self | rhs as u8
    }
}
