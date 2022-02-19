use std::ops::Bound;
use std::ops::Bound::*;

use crate::{Full, StartExclusive, StartInclusive, EndExclusive, EndInclusive, StartInclusiveEndInclusive, StartExclusiveEndInclusive, StartInclusiveEndExclusive, StartExclusiveEndExclusive};

pub trait Contain<T: ?Sized + PartialOrd> {
    /// start bound
    fn start_bound(&self) -> Bound<&T>;

    /// end bound
    fn end_bound(&self) -> Bound<&T>;

    /// Whether this range is null.
    fn is_null(&self) -> bool {
        match (self.start_bound(), self.end_bound()) {
            // Either bound is unbound, the range is not null
            (Unbounded, _) => false,
            (_, Unbounded) => false,
            // Both included
            (Included(start), Included(end)) => start > end,
            // Either bound is excluded
            (Excluded(start), Excluded(end)) => start >= end,
            (Included(start), Excluded(end)) => start >= end,
            (Excluded(start), Included(end)) => start >= end,
        }
    }

    // Relative position
    fn relative<U>(&self, item: &U) -> Option<Relative>
    where
        T: PartialOrd<U>,
        U: PartialOrd<T> + ?Sized,
    {
        if self.is_null() {
            return None;
        }

        let res = match (self.start_bound(), self.end_bound()) {
            (Included(start), _) if item < start => Relative::Below,
            (Excluded(start), _) if item <= start => Relative::Below,
            (_, Included(end)) if item > end => Relative::Above,
            (_, Excluded(end)) if item >= end => Relative::Above,
            _ => Relative::In,
        };
        Some(res)
    }

    /// Whether this range contains the item.
    fn contains<U>(&self, item: &U) -> bool
    where
        T: PartialOrd<U>,
        U: PartialOrd<T> + ?Sized,
    {
        self.relative(item).map(|rel| rel.is_in()).unwrap_or(false)
    }
}

/// Relative position
pub enum Relative {
    /// Below the range
    Below,
    /// In the range
    In,
    /// Above the range
    Above,
}

impl Relative {
    pub fn is_in(&self) -> bool {
        matches!(self, Relative::In)
    }

    pub fn is_below(&self) -> bool {
        matches!(self, Relative::Below)
    }

    pub fn is_above(&self) -> bool {
        matches!(self, Relative::Above)
    }
}

impl<T: PartialOrd + ?Sized> Contain<T> for Full {
    fn start_bound(&self) -> Bound<&T> {
        Unbounded
    }

    fn end_bound(&self) -> Bound<&T> {
        Unbounded
    }
}

impl<T: PartialOrd> Contain<T> for StartExclusive<T> {
    fn start_bound(&self) -> Bound<&T> {
        Excluded(&self.start)
    }

    fn end_bound(&self) -> Bound<&T> {
        Unbounded
    }
}

impl<T: PartialOrd> Contain<T> for StartInclusive<T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&T> {
        Unbounded
    }
}

impl<T: PartialOrd> Contain<T> for EndExclusive<T> {
    fn start_bound(&self) -> Bound<&T> {
        Unbounded
    }

    fn end_bound(&self) -> Bound<&T> {
        Excluded(&self.end)
    }
}

impl<T: PartialOrd> Contain<T> for EndInclusive<T> {
    fn start_bound(&self) -> Bound<&T> {
        Unbounded
    }

    fn end_bound(&self) -> Bound<&T> {
        Included(&self.end)
    }
}

impl<T: PartialOrd> Contain<T> for StartInclusiveEndInclusive<T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&T> {
        Included(&self.end)
    }
}

impl<T: PartialOrd> Contain<T> for StartExclusiveEndInclusive<T> {
    fn start_bound(&self) -> Bound<&T> {
        Excluded(&self.start)
    }

    fn end_bound(&self) -> Bound<&T> {
        Included(&self.end)
    }
}

impl<T: PartialOrd> Contain<T> for StartInclusiveEndExclusive<T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&T> {
        Excluded(&self.end)
    }
}

impl<T: PartialOrd> Contain<T> for StartExclusiveEndExclusive<T> {
    fn start_bound(&self) -> Bound<&T> {
        Excluded(&self.start)
    }

    fn end_bound(&self) -> Bound<&T> {
        Excluded(&self.end)
    }
}