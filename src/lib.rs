// SPDX-License-Identifier: CC0-1.0

//! Provides a wrapper for types that can technically implement `PartialOrd`/`Ord` but for semantic
//! reasons it is nonsensical.
//!
//! `PartialOrd` and `Ord` are often useful and/or required. For example, [`Ordered`] allows one to
//! use such a type as a key in a `BTreeMap` (which requires ordered keys).
//!
//! For a full example see [`examples/point.rs`].
//!
//! # Examples
//!
//! ```
//! # #![allow(unused)] // Because of `Adt`.
//! use core::cmp::Ordering;
//! use ordered::{ArbitraryOrd, Ordered};
//!
//! /// A point in 2D space.
//! ///
//! /// We do not want users to be able to write `a < b` because it is not well defined.
//! #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
//! struct Point {
//!     x: u32,
//!     y: u32,
//! }
//!
//! impl ArbitraryOrd for Point {
//!     fn arbitrary_cmp(&self, other: &Self) -> Ordering {
//!         // Just use whatever order tuple cmp gives us.
//!         (self.x, self.y).cmp(&(other.x, other.y))
//!     }
//! }
//!
//! /// `Ordered` allows users to derive `PartialOrd` on types that include a `Point`.
//! #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
//! struct Adt {
//!     name: String,
//!     point: Ordered<Point>,
//! }
//! ```
//!
//! [`examples/point.rs`]: <https://github.com/rust-bitcoin/rust-ordered/blob/master/examples/point.rs>

#![no_std]
// Experimental features we need.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Coding conventions.
#![warn(missing_docs)]
#![warn(deprecated_in_future)]
#![doc(test(attr(warn(unused))))]

use core::borrow::{Borrow, BorrowMut};
use core::cmp::Ordering;
use core::fmt;
use core::ops::{Deref, DerefMut};

/// Trait for types that perform an arbitrary ordering.
///
/// More specifically, this trait is for types that perform either a partial or
/// total order but semantically it is nonsensical.
///
/// # Examples
///
/// ```
/// # #![allow(unused)] // Because of `Adt`.
/// use core::cmp::Ordering;
/// use ordered::ArbitraryOrd;
///
/// /// A point in 2D space.
/// ///
/// /// We do not want users to be able to write `a < b` because it is not well defined.
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// struct Point {
///     x: u32,
///     y: u32,
/// }
///
/// impl ArbitraryOrd for Point {
///     fn arbitrary_cmp(&self, other: &Self) -> Ordering {
///         // Just use whatever order tuple cmp gives us.
///         (self.x, self.y).cmp(&(other.x, other.y))
///     }
/// }
/// ```
pub trait ArbitraryOrd<Rhs = Self>: PartialEq<Rhs> {
    /// Implements a meaningless, arbitrary ordering.
    fn arbitrary_cmp(&self, other: &Rhs) -> Ordering;
}

/// A wrapper type that implements `PartialOrd` and `Ord`.
///
/// # Examples
///
/// ```
/// # #![allow(unused)] // Because of `Adt`.
/// use core::cmp::Ordering;
/// use ordered::{ArbitraryOrd, Ordered};
///
/// /// A point in 2D space.
/// ///
/// /// We do not want users to be able to write `a < b` because it is not well defined.
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// struct Point {
///     x: u32,
///     y: u32,
/// }
///
/// impl ArbitraryOrd for Point {
///     fn arbitrary_cmp(&self, other: &Self) -> Ordering {
///         // Just use whatever order tuple cmp gives us.
///         (self.x, self.y).cmp(&(other.x, other.y))
///     }
/// }
///
/// let point = Point { x: 0, y: 1 };
/// let ordered = Ordered(point);
///
/// assert_eq!(*ordered, point); // Use `ops::Deref`.
/// assert_eq!(&ordered.0, ordered.as_ref()); // Use the public inner field or `AsRef`.
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Ordered<T>(pub T);

impl<T: Copy> Copy for Ordered<T> {}

impl<T> Ordered<T> {
    /// Creates a new wrapped ordered type.
    ///
    /// The inner type is public so this function is never explicitly needed.
    pub const fn new(inner: T) -> Self { Self(inner) }

    /// Creates an `Ordered<T>` from a reference.
    ///
    /// This allows: `let found = map.get(Ordered::from_ref(&a));`
    #[allow(clippy::ptr_as_ptr)]
    pub fn from_ref(value: &T) -> &Self { unsafe { &*(value as *const _ as *const Self) } }

    /// Returns a reference to the inner object.
    ///
    /// We also implement [`core::borrow::Borrow`] so this function is never explicitly needed.
    #[deprecated(since = "0.3.0", note = "use `ops::Deref` instead")]
    pub const fn as_inner(&self) -> &T { &self.0 }

    /// Returns the inner object.
    ///
    /// We also implement [`core::ops::Deref`] so this function is never explicitly needed.
    #[deprecated(since = "0.3.0", note = "use `ops::Deref` instead")]
    pub fn into_inner(self) -> T { self.0 }
}

impl<T: ArbitraryOrd> ArbitraryOrd for &T {
    fn arbitrary_cmp(&self, other: &Self) -> Ordering { (*self).arbitrary_cmp(other) }
}

impl<T: ArbitraryOrd> PartialOrd for Ordered<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some((*self).arbitrary_cmp(other)) }
}

impl<T: ArbitraryOrd + Eq> Ord for Ordered<T> {
    fn cmp(&self, other: &Self) -> Ordering { (*self).arbitrary_cmp(other) }
}

impl<T: fmt::Display> fmt::Display for Ordered<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Display::fmt(&self.0, f) }
}

impl<T> From<T> for Ordered<T> {
    fn from(inner: T) -> Self { Self(inner) }
}

impl<T> AsRef<T> for Ordered<T> {
    fn as_ref(&self) -> &T { &self.0 }
}

impl<T> AsMut<T> for Ordered<T> {
    fn as_mut(&mut self) -> &mut T { &mut self.0 }
}

impl<T> Borrow<T> for Ordered<T> {
    fn borrow(&self) -> &T { &self.0 }
}

impl<T> BorrowMut<T> for Ordered<T> {
    fn borrow_mut(&mut self) -> &mut T { &mut self.0 }
}

impl<T> Deref for Ordered<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> DerefMut for Ordered<T> {
    fn deref_mut(&mut self) -> &mut T { &mut self.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Point {
        x: u32,
        y: u32,
    }

    impl Point {
        fn new(x: u32, y: u32) -> Self { Point { x, y } }
    }

    impl ArbitraryOrd for Point {
        fn arbitrary_cmp(&self, other: &Self) -> Ordering {
            (self.x, self.y).cmp(&(other.x, other.y))
        }
    }

    #[test]
    fn can_compare() {
        let a = Point::new(2, 3);
        let b = Point::new(5, 7);

        assert!(Ordered(a) < Ordered(b));
    }

    #[test]
    fn can_compare_with_from_ref() {
        let a = Point::new(2, 3);
        let b = Point::new(5, 7);

        assert!(Ordered::from_ref(&a) < Ordered::from_ref(&b));
    }

    #[test]
    fn can_compare_with_reference() {
        let a = Point::new(2, 3);
        let b = Point::new(5, 7);

        assert!(Ordered(&a) < Ordered(&b));
    }

    // Copied from https://rust-lang.github.io/api-guidelines/interoperability.html#c-send-sync
    #[test]
    fn send() {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        struct Point {
            x: u32,
            y: u32,
        }

        impl ArbitraryOrd for Point {
            fn arbitrary_cmp(&self, other: &Self) -> Ordering {
                (self.x, self.y).cmp(&(other.x, other.y))
            }
        }

        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<Ordered<Point>>();
        assert_sync::<Ordered<Point>>();
    }

    #[test]
    fn trait_is_object_safe() {
        extern crate std;
        use std::boxed::Box;

        // If this test builds then `ArbitraryOrd` is object safe.
        #[allow(dead_code)]
        struct ObjectSafe {
            p: Box<dyn ArbitraryOrd<Self>>,
            q: Box<dyn PartialOrd<Self>>, // Sanity check.
        }
    }
}
