mod contain;
mod distance;
mod cut;
mod closest;

pub use contain::{Contain, Relative};
pub use distance::{Distance, RangeSize};
pub use cut::{AboveExclusive, AboveInclusive, BelowExclusive, BelowInclusive};
pub use std::ops::Bound;
pub use std::ops::Bound::*;

/// An unbounded range
pub struct Full;

/// A range only bounded exclusively below (start,)
pub struct StartExclusive<A> {
    pub start: A,
}

/// A range only bounded inclusively below [start,)
pub struct StartInclusive<A> {
    pub start: A,
}

/// A range only boudned exclusively beyond (,end)
pub struct EndExclusive<A> {
    pub end: A,
}

/// A range only bounded inclusively beyond (,end]
pub struct EndInclusive<A> {
    pub end: A,
}

/// A range bounded inclusively below and inclusively above [start, end]
pub struct StartInclusiveEndInclusive<A> {
    pub start: A,
    pub end: A,
}

/// A range bounded exclusively below and inclusively above (start, end]
pub struct StartExclusiveEndInclusive<A> {
    pub start: A,
    pub end: A,
}

/// A range bounded inclusively below and exclusively above [start, end]
pub struct StartInclusiveEndExclusive<A> {
    pub start: A,
    pub end: A,
}

/// A range bounded exclusively below and exclusively above (start, end]
pub struct StartExclusiveEndExclusive<A> {
    pub start: A,
    pub end: A,
}
