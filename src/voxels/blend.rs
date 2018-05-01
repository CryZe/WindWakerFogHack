use core::num::Float;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Operation {
    Max,
    Min,
    Add,
    Sub,
}

pub fn execute<B: Blend>(a: B, b: B, op: Operation) -> B {
    use self::Operation::*;

    match op {
        Max => a.max(b),
        Min => a.min(b),
        Add => a.add(b),
        Sub => a.sub(b),
    }
}

pub trait Blend {
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
}

impl Blend for f32 {
    fn max(self, other: Self) -> Self {
        Float::max(self, other)
    }

    fn min(self, other: Self) -> Self {
        Float::min(self, 1.0 - other)
    }

    fn add(self, other: Self) -> Self {
        Float::min(self + other, 1.0)
    }

    fn sub(self, other: Self) -> Self {
        Float::max(self - other, 0.0)
    }
}
