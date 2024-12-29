//! Showcase using `Ordered` with an unordered type as a `BTreeMap` key.

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;

use ordered::{ArbitraryOrd, Ordered};

fn main() {
    let mut map = BTreeMap::new();

    let a = Point { x: 2, y: 3 };
    let b = Point { x: 1, y: 5 };

    map.insert(Ordered::from(a), "some interesting value");
    map.insert(Ordered::from(b), "some other interesting value");

    println!();
    println!("Looking in map for key: {}", a);
    let found = map
        .get(Ordered::from_ref(&a))
        .expect("failed to look up key");
    println!("Found it, with value: {}", found);
}

/// A point in 2D space.
///
/// We do not want users to be able to write `a < b` because it is not well defined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl ArbitraryOrd for Point {
    fn arbitrary_cmp(&self, other: &Self) -> Ordering { (self.x, self.y).cmp(&(other.x, other.y)) }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "({}, {})", self.x, self.y) }
}
