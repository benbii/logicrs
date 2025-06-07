use std::{
    fmt::Debug,
    ops::{BitAnd, BitOr, Not},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Lbool(pub u8);

impl Lbool {
    pub const FALSE: Lbool = Lbool(0);
    pub const TRUE: Lbool = Lbool(1);
    pub const NONE: Lbool = Lbool(2);

    #[inline]
    pub fn is_true(self) -> bool {
        self == Self::TRUE
    }

    #[inline]
    pub fn is_false(self) -> bool {
        self == Self::FALSE
    }

    #[inline]
    pub fn is_none(self) -> bool {
        self.0 & 2 != 0
    }

    #[inline]
    pub fn not_if(self, x: bool) -> Self {
        if x { !self } else { self }
    }
}

impl From<bool> for Lbool {
    #[inline]
    fn from(value: bool) -> Self {
        Self(value as u8)
    }
}

impl From<Lbool> for Option<bool> {
    #[inline]
    fn from(val: Lbool) -> Self {
        match val {
            Lbool::TRUE => Some(true),
            Lbool::FALSE => Some(false),
            _ => None,
        }
    }
}

impl Default for Lbool {
    #[inline]
    fn default() -> Self {
        Self::NONE
    }
}

impl Debug for Lbool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let field = match *self {
            Lbool::TRUE => Some(true),
            Lbool::FALSE => Some(false),
            _ => None,
        };
        f.debug_tuple("Lbool").field(&field).finish()
    }
}

impl Not for Lbool {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        Lbool(self.0 ^ 1)
    }
}

impl BitAnd for Lbool {
    type Output = Lbool;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        if self.is_none() {
            if rhs.is_false() {
                Self::FALSE
            } else {
                Self::NONE
            }
        } else if rhs.is_none() {
            if self.is_false() {
                Self::FALSE
            } else {
                Self::NONE
            }
        } else {
            Self(self.0 & rhs.0)
        }
    }
}

impl BitOr for Lbool {
    type Output = Lbool;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        if self.is_none() {
            if rhs.is_true() {
                Self::TRUE
            } else {
                Self::NONE
            }
        } else if rhs.is_none() {
            if self.is_true() {
                Self::TRUE
            } else {
                Self::NONE
            }
        } else {
            Self(self.0 | rhs.0)
        }
    }
}
