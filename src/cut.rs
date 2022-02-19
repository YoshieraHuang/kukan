use crate::contain::Contain;
use crate::{Full, StartExclusive, StartInclusive, EndExclusive, EndInclusive, StartInclusiveEndInclusive, StartExclusiveEndInclusive, StartInclusiveEndExclusive, StartExclusiveEndExclusive};

/// Cut the range and give the below inclusive range.
pub trait BelowInclusive<T: PartialOrd> {
    /// Below range
    type Range: Contain<T>;

    /// Inclusive below range
    fn below_inclusive(self, edge: T) -> Self::Range;
}

/// Cut the range and give the below exclusive range.
pub trait BelowExclusive<T: PartialOrd> {
    /// Below range
    type Range: Contain<T>;

    /// Inclusive below range
    fn below_exclusive(self, edge: T) -> Self::Range;
}

/// Cut the range and give the above inclusive range.
pub trait AboveInclusive<T: PartialOrd> {
    /// Above range
    type Range: Contain<T>;

    /// Above inclusive range
    fn above_inclusive(self, edge: T) -> Self::Range;
}

/// Cut the range and give the above exclusive range.
pub trait AboveExclusive<T: PartialOrd> {
    /// Above range
    type Range: Contain<T>;

    /// Above exclusive range
    fn above_exclusive(self, edge: T) -> Self::Range;
}

impl<T: PartialOrd> BelowInclusive<T> for Full {
    type Range = EndInclusive<T>;

    fn below_inclusive(self, edge: T) -> Self::Range {
        Self::Range { end: edge }
    }
}

impl<T: PartialOrd> AboveExclusive<T> for Full {
    type Range = StartExclusive<T>;

    fn above_exclusive(self, edge: T) -> Self::Range {
        Self::Range { start: edge }
    }
}

impl<T: PartialOrd> BelowExclusive<T> for Full {
    type Range = EndExclusive<T>;

    fn below_exclusive(self, edge: T) -> Self::Range {
        Self::Range { end: edge }
    }
}

impl<T: PartialOrd> AboveInclusive<T> for Full {
    type Range = StartInclusive<T>;

    fn above_inclusive(self, edge: T) -> Self::Range {
        Self::Range { start: edge }
    }
}

macro_rules! impl_cut_for_end {
    ($range: ident -> below: $trait: ident, $fn: ident, $range_ty: ident) => {
        impl<T: Ord> $trait<T> for $range<T> {
            type Range = $range_ty<T>;

            fn $fn(self, edge: T) -> Self::Range {
                Self::Range {
                    end: std::cmp::min(edge, self.end)
                }
            }
        }
    };
    ($range: ident -> beyond: $trait: ident, $fn: ident, $range_ty: ident) => {
        impl<T: Ord> $trait<T> for $range<T> {
            type Range = $range_ty<T>;

            fn $fn(self, edge: T) -> Self::Range {
                Self::Range {
                    start: edge,
                    end: self.end
                }
            }
        }
    };
    ($range: ident, $below_in_range: ident, $below_ex_range: ident, $beyond_in_range: ident, $beyond_ex_range: ident) => {
        impl_cut_for_end!($range -> below: BelowInclusive, below_inclusive, $below_in_range);
        impl_cut_for_end!($range -> below: BelowExclusive, below_exclusive, $below_ex_range);
        impl_cut_for_end!($range -> beyond: AboveInclusive, above_inclusive, $beyond_in_range);
        impl_cut_for_end!($range -> beyond: AboveExclusive, above_exclusive, $beyond_ex_range);
    }
}

impl_cut_for_end!(
    EndExclusive,
    EndInclusive,
    EndExclusive,
    StartInclusiveEndExclusive,
    StartExclusiveEndExclusive
);
impl_cut_for_end!(
    EndInclusive,
    EndInclusive,
    EndExclusive,
    StartInclusiveEndInclusive,
    StartExclusiveEndInclusive
);

macro_rules! impl_cut_for_start {
    ($range: ident -> below: $trait: ident, $fn: ident, $range_ty: ident) => {
        impl<T: Ord> $trait<T> for $range<T> {
            type Range = $range_ty<T>;

            fn $fn(self, edge: T) -> Self::Range {
                Self::Range {
                    start: self.start,
                    end: edge,
                }
            }
        }
    };
    ($range: ident -> beyond: $trait: ident, $fn: ident, $range_ty: ident) => {
        impl<T: Ord> $trait<T> for $range<T> {
            type Range = $range_ty<T>;

            fn $fn(self, edge: T) -> Self::Range {
                Self::Range {
                    start: std::cmp::max(self.start, edge)
                }
            }
        }
    };
    ($range: ident, $below_in_range: ident, $below_ex_range: ident, $beyond_in_range: ident, $beyond_ex_range: ident) => {
        impl_cut_for_start!($range -> below: BelowInclusive, below_inclusive, $below_in_range);
        impl_cut_for_start!($range -> below: BelowExclusive, below_exclusive, $below_ex_range);
        impl_cut_for_start!($range -> beyond: AboveInclusive, above_inclusive, $beyond_in_range);
        impl_cut_for_start!($range -> beyond: AboveExclusive, above_exclusive, $beyond_ex_range);
    }
}

impl_cut_for_start!(
    StartExclusive,
    StartExclusiveEndInclusive,
    StartExclusiveEndExclusive,
    StartInclusive,
    StartExclusive
);
impl_cut_for_start!(
    StartInclusive,
    StartInclusiveEndInclusive,
    StartInclusiveEndExclusive,
    StartInclusive,
    StartExclusive
);

macro_rules! impl_cut_for_two_end {
    ($range: ident -> below: $trait: ident, $fn: ident, $range_ty: ident) => {
        impl<T: Ord> $trait<T> for $range<T> {
            type Range = $range_ty<T>;

            fn $fn(self, edge: T) -> Self::Range {
                Self::Range {
                    start: self.start,
                    end: std::cmp::min(edge, self.end),
                }
            }
        }
    };
    ($range: ident -> beyond: $trait: ident, $fn: ident, $range_ty: ident) => {
        impl<T: Ord> $trait<T> for $range<T> {
            type Range = $range_ty<T>;

            fn $fn(self, edge: T) -> Self::Range {
                Self::Range {
                    start: std::cmp::max(self.start, edge),
                    end: self.end,
                }
            }
        }
    };
    ($range: ident, $below_in_range: ident, $below_ex_range: ident, $beyond_in_range: ident, $beyond_ex_range: ident) => {
        impl_cut_for_two_end!($range -> below: BelowInclusive, below_inclusive, $below_in_range);
        impl_cut_for_two_end!($range -> below: BelowExclusive, below_exclusive, $below_ex_range);
        impl_cut_for_two_end!($range -> beyond: AboveInclusive, above_inclusive, $beyond_in_range);
        impl_cut_for_two_end!($range -> beyond: AboveExclusive, above_exclusive, $beyond_ex_range);
    }
}

impl_cut_for_two_end!(
    StartInclusiveEndInclusive,
    StartInclusiveEndInclusive,
    StartInclusiveEndExclusive,
    StartInclusiveEndInclusive,
    StartExclusiveEndInclusive
);
impl_cut_for_two_end!(
    StartExclusiveEndInclusive,
    StartExclusiveEndInclusive,
    StartExclusiveEndExclusive,
    StartInclusiveEndInclusive,
    StartExclusiveEndInclusive
);
impl_cut_for_two_end!(
    StartInclusiveEndExclusive,
    StartInclusiveEndInclusive,
    StartInclusiveEndExclusive,
    StartInclusiveEndExclusive,
    StartExclusiveEndExclusive
);
impl_cut_for_two_end!(
    StartExclusiveEndExclusive,
    StartExclusiveEndInclusive,
    StartExclusiveEndExclusive,
    StartInclusiveEndExclusive,
    StartExclusiveEndExclusive
);
