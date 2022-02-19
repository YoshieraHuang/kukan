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

/// Range Size
pub trait RangeSize<T>: Contain<T>
where
    T: PartialOrd,
{
    /// Range Size
    fn range_size(&self) -> Option<T::Scalar>
    where
        T: Copy + Zero + Distance,
        T::Scalar: Zero,
    {
        if self.is_null() {
            return Some(T::Scalar::zero());
        }

        match (self.start_bound(), self.end_bound()) {
            (Unbounded, _) => None,
            (_, Unbounded) => None,
            (Included(start), Included(end)) => Some(end.distance(*start)),
            (Excluded(start), Included(end)) => Some(end.distance(*start)),
            (Included(start), Excluded(end)) => Some(end.distance(*start)),
            (Excluded(start), Excluded(end)) => Some(end.distance(*start)),
        }
    }
}

impl<T, Range> RangeSize<T> for Range
where
    Range: Contain<T>,
    T: Copy + PartialOrd + Zero + Distance,
    T::Scalar: Zero,
{
}
