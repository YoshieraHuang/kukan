use crate::contain::{Contain, Relative};
use std::ops::Bound::*;

/// Closest value
pub trait ClosestValue<T>: Contain<T>
where
    T: PartialOrd,
{
    /// Return the closest value within the range to the given value
    fn closest(&self, t: T) -> Option<T>
    where
        T: Copy,
    {
        self.relative(&t).and_then(|rel| match rel {
            Relative::In => Some(t),
            Relative::Below => match self.left_bound() {
                Included(left) => Some(*left),
                Excluded(_) => None,
                Unbounded => unreachable!(),
            },
            Relative::Above => match self.right_bound() {
                Included(right) => Some(*right),
                Excluded(_) => None,
                Unbounded => unreachable!(),
            },
        })
    }
}

impl<T, Range> ClosestValue<T> for Range
where
    Range: Contain<T>,
    T: PartialOrd + Copy
{}