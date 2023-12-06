// SPDX-License-Identifier: CC0-1.0

//! Provides a wrapper for types that can technically implement `PartialOrd`/`Ord`
//! but for semantic reasons it is nonsensical.
//!
//! # Examples
//!
//! ```
//! use std::cmp::Ordering;
//! use std::collections::BTreeMap;
//! use ordered::{ArbitraryOrd, Ordered};
//!
//! /// A Foo type.
//! ///
//! /// We do not want users to be able to write `a < b` because it is meaningless
//! /// to compare the two but we wish to use `Foo`, for example, as a `BTreeMap` key.
//! #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
//! enum Foo {
//!     /// A space foo.
//!     Space(u32),
//!     /// A time foo.
//!     Time(u32)
//! }
//!
//! impl ArbitraryOrd for Foo {
//!     fn arbitrary_cmp(&self, other: &Self) -> Ordering {
//!         use Foo::*;
//!         match (self, other) {
//!             (Space(_), Time(_)) => Ordering::Less,
//!             (Time(_), Space(_)) => Ordering::Greater,
//!             (Space(this), Space(that)) => this.cmp(that),
//!             (Time(this), Time(that)) => this.cmp(that),
//!         }
//!     }
//! }
//!
//! let a = Foo::Space(50);
//! let b = Foo::Time(50);
//!
//! let mut map = BTreeMap::new();
//!
//! // error[E0277]: the trait bound `Foo: Ord` is not satisfied
//! // map.insert(a, "some interesting value");
//!
//! map.insert(Ordered(a), "some interesting value");
//! map.insert(Ordered(b), "some other interesting value");
//!
//! println!("Looking in map for key: {}", a);
//! let found = map.get(&Ordered::from(a)).expect("failed to look up key");
//! println!("With value: {}", found);
//! ```

#![no_std]
// Experimental features we need.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Coding conventions.
#![warn(missing_docs)]

use core::borrow::{Borrow, BorrowMut};
use core::cmp::Ordering;
use core::fmt;
use core::ops::{Deref, DerefMut};

/// Trait for types that perform an arbitrary ordering.
///
/// More specifically, this trait is for types that perform either a partial or
/// total order but semantically it is nonsensical.
pub trait ArbitraryOrd: Eq + PartialEq {
    /// Implements a meaningless, arbitrary ordering.
    fn arbitrary_cmp(&self, other: &Self) -> Ordering;
}

/// A wrapper type that implements `PartialOrd` and `Ord`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ordered<T>(pub T);

impl<T: Copy> Copy for Ordered<T> {}

impl<T> Ordered<T> {
    /// Creates a new wrapped ordered type.
    ///
    /// The inner type is public so this function is never explicitly needed.
    pub fn new(inner: T) -> Self { Self(inner) }

    /// Returns a reference to the inner object.
    ///
    /// We also implement [`core::borrow::Borrow`] so this function is never explicitly needed.
    pub fn as_inner(&self) -> &T { &self.0 }

    /// Returns the inner object.
    ///
    /// We also implement [`core::ops::Deref`] so this function is never explicitly needed.
    pub fn into_inner(self) -> T { self.0 }
}

impl<T: ArbitraryOrd> PartialOrd for Ordered<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl<T: ArbitraryOrd> Ord for Ordered<T> {
    fn cmp(&self, other: &Self) -> Ordering { self.0.arbitrary_cmp(&other.0) }
}

impl<T: fmt::Display> fmt::Display for Ordered<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Display::fmt(&self.0, f) }
}

impl<T> From<T> for Ordered<T> {
    fn from(inner: T) -> Self { Self(inner) }
}

impl<T> Deref for Ordered<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> DerefMut for Ordered<T> {
    fn deref_mut(&mut self) -> &mut T { &mut self.0 }
}

impl<T> Borrow<T> for Ordered<T> {
    fn borrow(&self) -> &T { &self.0 }
}

impl<T> BorrowMut<T> for Ordered<T> {
    fn borrow_mut(&mut self) -> &mut T { &mut self.0 }
}
