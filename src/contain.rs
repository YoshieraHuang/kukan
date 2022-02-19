use std::ops::Bound;
use std::ops::Bound::*;

use crate::{Full, LeftOpen, LeftClosed, RightOpen, RightClosed, LeftClosedRightClosed, LeftOpenRightClosed, LeftClosedRightOpen, LeftOpenRightOpen};

pub trait Contain<T: ?Sized + PartialOrd> {
    /// left bound
    fn left_bound(&self) -> Bound<&T>;

    /// right bound
    fn right_bound(&self) -> Bound<&T>;

    /// Whether this interval is null.
    fn is_null(&self) -> bool {
        match (self.left_bound(), self.right_bound()) {
            // Either bound is unbound, the interval is not null
            (Unbounded, _) => false,
            (_, Unbounded) => false,
            // Both included
            (Included(left), Included(right)) => left > right,
            // Either bound is excluded
            (Excluded(left), Excluded(right)) => left >= right,
            (Included(left), Excluded(right)) => left >= right,
            (Excluded(left), Included(right)) => left >= right,
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

        let res = match (self.left_bound(), self.right_bound()) {
            (Included(left), _) if item < left => Relative::Below,
            (Excluded(left), _) if item <= left => Relative::Below,
            (_, Included(right)) if item > right => Relative::Above,
            (_, Excluded(right)) if item >= right => Relative::Above,
            _ => Relative::In,
        };
        Some(res)
    }

    /// Whether this interval contains the item.
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
    /// Below the interval
    Below,
    /// In the interval
    In,
    /// Above the interval
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
    fn left_bound(&self) -> Bound<&T> {
        Unbounded
    }

    fn right_bound(&self) -> Bound<&T> {
        Unbounded
    }
}

impl<T: PartialOrd> Contain<T> for LeftOpen<T> {
    fn left_bound(&self) -> Bound<&T> {
        Excluded(&self.left)
    }

    fn right_bound(&self) -> Bound<&T> {
        Unbounded
    }
}

impl<T: PartialOrd> Contain<T> for LeftClosed<T> {
    fn left_bound(&self) -> Bound<&T> {
        Included(&self.left)
    }

    fn right_bound(&self) -> Bound<&T> {
        Unbounded
    }
}

impl<T: PartialOrd> Contain<T> for RightOpen<T> {
    fn left_bound(&self) -> Bound<&T> {
        Unbounded
    }

    fn right_bound(&self) -> Bound<&T> {
        Excluded(&self.right)
    }
}

impl<T: PartialOrd> Contain<T> for RightClosed<T> {
    fn left_bound(&self) -> Bound<&T> {
        Unbounded
    }

    fn right_bound(&self) -> Bound<&T> {
        Included(&self.right)
    }
}

impl<T: PartialOrd> Contain<T> for LeftClosedRightClosed<T> {
    fn left_bound(&self) -> Bound<&T> {
        Included(&self.left)
    }

    fn right_bound(&self) -> Bound<&T> {
        Included(&self.right)
    }
}

impl<T: PartialOrd> Contain<T> for LeftOpenRightClosed<T> {
    fn left_bound(&self) -> Bound<&T> {
        Excluded(&self.left)
    }

    fn right_bound(&self) -> Bound<&T> {
        Included(&self.right)
    }
}

impl<T: PartialOrd> Contain<T> for LeftClosedRightOpen<T> {
    fn left_bound(&self) -> Bound<&T> {
        Included(&self.left)
    }

    fn right_bound(&self) -> Bound<&T> {
        Excluded(&self.right)
    }
}

impl<T: PartialOrd> Contain<T> for LeftOpenRightOpen<T> {
    fn left_bound(&self) -> Bound<&T> {
        Excluded(&self.left)
    }

    fn right_bound(&self) -> Bound<&T> {
        Excluded(&self.right)
    }
}