use crate::mm::PAGE_SIZE;
pub const MAX_ORDER: usize = 10;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Order {
    #[default]
    Zero = 0,
    One = 1,
    Tow = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Eleven = 11,
    Twelve = 12,
    Thirteen = 13,
    Fourteen = 14,
    Fifteen = 15,
    Sixteen = 16,
}

impl Order {
    #[inline]
    pub const fn as_size(self) -> u32 {
        1 << (self as u32 + PAGE_SIZE.trailing_zeros())
    }

    #[inline]
    pub const fn as_power(self) -> u8 {
        self as u8 + PAGE_SIZE.trailing_zeros() as u8
    }

    #[inline]
    pub const fn max_order() -> Self {
        Self::Ten
    }

    #[inline]
    pub const fn min_order() -> Self {
        Self::Zero
    }

    #[inline]
    pub const fn from_order(order: usize) -> Self {
        match order {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Tow,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            10 => Self::Ten,
            11 => Self::Eleven,
            12 => Self::Twelve,
            13 => Self::Thirteen,
            14 => Self::Fourteen,
            15 => Self::Fifteen,
            16 => Self::Sixteen,
            _ => panic!("Buddy Order invalid!"),
        }
    }
}
