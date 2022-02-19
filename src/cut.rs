use crate::contain::Contain;
use crate::{Full, LeftOpen, LeftClosed, RightOpen, RightClosed, LeftClosedRightClosed, LeftOpenRightClosed, LeftClosedRightOpen, LeftOpenRightOpen};

/// Cut the interval and give the below inclusive interval.
pub trait BelowInclusive<T: PartialOrd> {
    /// Below interval
    type Interval: Contain<T>;

    /// Inclusive below interval
    fn below_inclusive(self, edge: T) -> Self::Interval;
}

/// Cut the interval and give the below exclusive interval.
pub trait BelowExclusive<T: PartialOrd> {
    /// Below interval
    type Interval: Contain<T>;

    /// Inclusive below interval
    fn below_exclusive(self, edge: T) -> Self::Interval;
}

/// Cut the interval and give the above inclusive interval.
pub trait AboveInclusive<T: PartialOrd> {
    /// Above interval
    type Interval: Contain<T>;

    /// Above inclusive interval
    fn above_inclusive(self, edge: T) -> Self::Interval;
}

/// Cut the interval and give the above exclusive interval.
pub trait AboveExclusive<T: PartialOrd> {
    /// Above interval
    type Interval: Contain<T>;

    /// Above exclusive interval
    fn above_exclusive(self, edge: T) -> Self::Interval;
}

impl<T: PartialOrd> BelowInclusive<T> for Full {
    type Interval = RightClosed<T>;

    fn below_inclusive(self, edge: T) -> Self::Interval {
        Self::Interval { right: edge }
    }
}

impl<T: PartialOrd> AboveExclusive<T> for Full {
    type Interval = LeftOpen<T>;

    fn above_exclusive(self, edge: T) -> Self::Interval {
        Self::Interval { left: edge }
    }
}

impl<T: PartialOrd> BelowExclusive<T> for Full {
    type Interval = RightOpen<T>;

    fn below_exclusive(self, edge: T) -> Self::Interval {
        Self::Interval { right: edge }
    }
}

impl<T: PartialOrd> AboveInclusive<T> for Full {
    type Interval = LeftClosed<T>;

    fn above_inclusive(self, edge: T) -> Self::Interval {
        Self::Interval { left: edge }
    }
}

macro_rules! impl_cut_for_right {
    ($interval: ident -> below: $trait: ident, $fn: ident, $interval_ty: ident) => {
        impl<T: Ord> $trait<T> for $interval<T> {
            type Interval = $interval_ty<T>;

            fn $fn(self, edge: T) -> Self::Interval {
                Self::Interval {
                    right: std::cmp::min(edge, self.right)
                }
            }
        }
    };
    ($interval: ident -> beyond: $trait: ident, $fn: ident, $interval_ty: ident) => {
        impl<T: Ord> $trait<T> for $interval<T> {
            type Interval = $interval_ty<T>;

            fn $fn(self, edge: T) -> Self::Interval {
                Self::Interval {
                    left: edge,
                    right: self.right
                }
            }
        }
    };
    ($interval: ident, $below_in_interval: ident, $below_ex_interval: ident, $beyond_in_interval: ident, $beyond_ex_interval: ident) => {
        impl_cut_for_right!($interval -> below: BelowInclusive, below_inclusive, $below_in_interval);
        impl_cut_for_right!($interval -> below: BelowExclusive, below_exclusive, $below_ex_interval);
        impl_cut_for_right!($interval -> beyond: AboveInclusive, above_inclusive, $beyond_in_interval);
        impl_cut_for_right!($interval -> beyond: AboveExclusive, above_exclusive, $beyond_ex_interval);
    }
}

impl_cut_for_right!(
    RightOpen,
    RightClosed,
    RightOpen,
    LeftClosedRightOpen,
    LeftOpenRightOpen
);
impl_cut_for_right!(
    RightClosed,
    RightClosed,
    RightOpen,
    LeftClosedRightClosed,
    LeftOpenRightClosed
);

macro_rules! impl_cut_for_left {
    ($interval: ident -> below: $trait: ident, $fn: ident, $interval_ty: ident) => {
        impl<T: Ord> $trait<T> for $interval<T> {
            type Interval = $interval_ty<T>;

            fn $fn(self, edge: T) -> Self::Interval {
                Self::Interval {
                    left: self.left,
                    right: edge,
                }
            }
        }
    };
    ($interval: ident -> beyond: $trait: ident, $fn: ident, $interval_ty: ident) => {
        impl<T: Ord> $trait<T> for $interval<T> {
            type Interval = $interval_ty<T>;

            fn $fn(self, edge: T) -> Self::Interval {
                Self::Interval {
                    left: std::cmp::max(self.left, edge)
                }
            }
        }
    };
    ($interval: ident, $below_in_interval: ident, $below_ex_interval: ident, $beyond_in_interval: ident, $beyond_ex_interval: ident) => {
        impl_cut_for_left!($interval -> below: BelowInclusive, below_inclusive, $below_in_interval);
        impl_cut_for_left!($interval -> below: BelowExclusive, below_exclusive, $below_ex_interval);
        impl_cut_for_left!($interval -> beyond: AboveInclusive, above_inclusive, $beyond_in_interval);
        impl_cut_for_left!($interval -> beyond: AboveExclusive, above_exclusive, $beyond_ex_interval);
    }
}

impl_cut_for_left!(
    LeftOpen,
    LeftOpenRightClosed,
    LeftOpenRightOpen,
    LeftClosed,
    LeftOpen
);
impl_cut_for_left!(
    LeftClosed,
    LeftClosedRightClosed,
    LeftClosedRightOpen,
    LeftClosed,
    LeftOpen
);

macro_rules! impl_cut_for_two_right {
    ($interval: ident -> below: $trait: ident, $fn: ident, $interval_ty: ident) => {
        impl<T: Ord> $trait<T> for $interval<T> {
            type Interval = $interval_ty<T>;

            fn $fn(self, edge: T) -> Self::Interval {
                Self::Interval {
                    left: self.left,
                    right: std::cmp::min(edge, self.right),
                }
            }
        }
    };
    ($interval: ident -> beyond: $trait: ident, $fn: ident, $interval_ty: ident) => {
        impl<T: Ord> $trait<T> for $interval<T> {
            type Interval = $interval_ty<T>;

            fn $fn(self, edge: T) -> Self::Interval {
                Self::Interval {
                    left: std::cmp::max(self.left, edge),
                    right: self.right,
                }
            }
        }
    };
    ($interval: ident, $below_in_interval: ident, $below_ex_interval: ident, $beyond_in_interval: ident, $beyond_ex_interval: ident) => {
        impl_cut_for_two_right!($interval -> below: BelowInclusive, below_inclusive, $below_in_interval);
        impl_cut_for_two_right!($interval -> below: BelowExclusive, below_exclusive, $below_ex_interval);
        impl_cut_for_two_right!($interval -> beyond: AboveInclusive, above_inclusive, $beyond_in_interval);
        impl_cut_for_two_right!($interval -> beyond: AboveExclusive, above_exclusive, $beyond_ex_interval);
    }
}

impl_cut_for_two_right!(
    LeftClosedRightClosed,
    LeftClosedRightClosed,
    LeftClosedRightOpen,
    LeftClosedRightClosed,
    LeftOpenRightClosed
);
impl_cut_for_two_right!(
    LeftOpenRightClosed,
    LeftOpenRightClosed,
    LeftOpenRightOpen,
    LeftClosedRightClosed,
    LeftOpenRightClosed
);
impl_cut_for_two_right!(
    LeftClosedRightOpen,
    LeftClosedRightClosed,
    LeftClosedRightOpen,
    LeftClosedRightOpen,
    LeftOpenRightOpen
);
impl_cut_for_two_right!(
    LeftOpenRightOpen,
    LeftOpenRightClosed,
    LeftOpenRightOpen,
    LeftClosedRightOpen,
    LeftOpenRightOpen
);
