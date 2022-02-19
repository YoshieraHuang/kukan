use crate::contain::Contain;
use num_traits::Zero;
use std::ops::Bound::*;

/// Distance between
pub trait Distance {
    type Scalar;

    /// distance
    fn distance(self, other: Self) -> Self::Scalar;
}

impl Distance for f32
{
    type Scalar = f32;

    fn distance(self, other: Self) -> Self::Scalar {
        (self - other).abs()
    }
}

impl Distance for f64
{
    type Scalar = f64;

    fn distance(self, other: Self) -> Self::Scalar {
        (self - other).abs()
    }
}

/// Interval size
pub trait IntervalSize<T>: Contain<T>
where
    T: PartialOrd,
{
    /// Interval size
    fn size(&self) -> Option<T::Scalar>
    where
        T: Copy + Zero + Distance,
        T::Scalar: Zero,
    {
        if self.is_null() {
            return Some(T::Scalar::zero());
        }

        match (self.left_bound(), self.right_bound()) {
            (Unbounded, _) => None,
            (_, Unbounded) => None,
            (Included(left), Included(right)) => Some(right.distance(*left)),
            (Excluded(left), Included(right)) => Some(right.distance(*left)),
            (Included(left), Excluded(right)) => Some(right.distance(*left)),
            (Excluded(left), Excluded(right)) => Some(right.distance(*left)),
        }
    }
}

impl<T, Range> IntervalSize<T> for Range
where
    Range: Contain<T>,
    T: Copy + PartialOrd + Zero + Distance,
    T::Scalar: Zero,
{
}
