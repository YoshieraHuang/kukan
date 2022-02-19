mod contain;
mod distance;
mod cut;
mod closest;

pub use contain::{Contain, Relative};
pub use distance::{Distance, IntervalSize};
pub use cut::{AboveExclusive, AboveInclusive, BelowExclusive, BelowInclusive};
pub use closest::ClosestValue;
pub use std::ops::Bound;
pub use std::ops::Bound::*;

/// An unbounded interval
pub struct Full;

/// A left-bounded open interval (left,)
pub struct LeftOpen<A> {
    pub left: A,
}

/// A left-bounded closed interval [left,)
pub struct LeftClosed<A> {
    pub left: A,
}

/// A right-bounded open interval (,right)
pub struct RightOpen<A> {
    pub right: A,
}

/// A right-bounded closed interval (,right]
pub struct RightClosed<A> {
    pub right: A,
}

/// An interval with left closed and right closed [left, right]
pub struct LeftClosedRightClosed<A> {
    pub left: A,
    pub right: A,
}

/// An interval with left open and right closed (left, right]
pub struct LeftOpenRightClosed<A> {
    pub left: A,
    pub right: A,
}

/// An interval with left closed and right open [left, right)
pub struct LeftClosedRightOpen<A> {
    pub left: A,
    pub right: A,
}

/// An interval with left open and right open (left, right)
pub struct LeftOpenRightOpen<A> {
    pub left: A,
    pub right: A,
}
