use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinState {
    High,
    Low,
    Floating,
    Conflicting,
    Undefined,
}

impl PinState {
    pub fn is_real(&self) -> bool {
        match self {
            PinState::High | PinState::Low => true,
            _ => false,
        }
    }

    pub fn is_error(&self) -> bool {
        match self {
            PinState::Conflicting | PinState::Undefined => true,
            _ => false,
        }
    }
}

impl Not for PinState {
    type Output = Self;

    fn not(self) -> Self::Output {
        use PinState::*;
        match self {
            High => Low,
            Low => High,
            _ => Undefined,
        }
    }
}

impl BitAnd for PinState {
    type Output = PinState;

    fn bitand(self, rhs: Self) -> Self::Output {
        use PinState::*;
        match (self, rhs) {
            (High, High) => High,
            (l, r) if l.is_real() && r.is_real() => Low,
            _ => Undefined,
        }
    }
}

impl BitAndAssign for PinState {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}

impl BitOr for PinState {
    type Output = PinState;

    fn bitor(self, rhs: Self) -> Self::Output {
        use PinState::*;
        match (self, rhs) {
            (Low, Low) => Low,
            (l, r) if l.is_real() && r.is_real() => High,
            _ => Undefined,
        }
    }
}

impl BitOrAssign for PinState {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl BitXor for PinState {
    type Output = PinState;

    fn bitxor(self, rhs: Self) -> Self::Output {
        use PinState::*;
        match (self, rhs) {
            (High, Low) | (Low, High) => High,
            (l, r) if l.is_real() && r.is_real() => Low,
            _ => Undefined,
        }
    }
}

impl BitXorAssign for PinState {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs
    }
}
