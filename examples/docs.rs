//! Code used in docs, crate level and README.md

use std::cmp::Ordering;
use std::fmt;
use std::collections::BTreeMap;
use ordered::{ArbitraryOrd, Ordered};

fn main() {
    b_tree_map();
    derive_and_access();
}

/// An example using a `Foo` as key to a `BTreeMap`.
fn b_tree_map() {
    let mut map = BTreeMap::new();

    let a = Foo::Space(50);
    let b = Foo::Time(50);

    // error[E0277]: the trait bound `Foo: Ord` is not satisfied
    // map.insert(a, "some interesting value");

    map.insert(Ordered(a), "some interesting value");
    map.insert(Ordered(b), "some other interesting value");
}

/// A Foo type.
///
/// We do not want users to be able to write `a < b` because it is meaningless
/// to compare the two but we wish to use `Foo`, for example, as a `BTreeMap` key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Foo {
    /// A space foo.
    Space(u32),
    /// A time foo.
    Time(u32)
}

impl ArbitraryOrd for Foo {
    fn arbitrary_cmp(&self, other: &Self) -> Ordering {
        use Foo::*;
        match (self, other) {
            (Space(_), Time(_)) => Ordering::Less,
            (Time(_), Space(_)) => Ordering::Greater,
            (Space(this), Space(that)) => this.cmp(that),
            (Time(this), Time(that)) => this.cmp(that),
        }
    }
}

impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Foo::Space(x) => write!(f, "Space {}", x),
            Foo::Time(x) => write!(f, "Time {}", x),
        }
    }
}

/// An example wrapping a non-ord type so we can derive `PartialOrd` and `Ord`.
///
/// Shows various ways to access the inner data.
fn derive_and_access() {
    let adt = Adt { x: 42, p: Foo::Space(50).into() };

    println!("We can explicitly deref: {}", *adt.p);
    println!("Or use deref coercion: {}", adt.p);
    println!("Or we can use borrow: {}", &adt.p);

    // And if all that is too complicated just use the inherent methods:
    println!("Explicitly get a reference: {}", adt.p.as_inner());
    println!("Or the inner type: {}", adt.p.into_inner());
}

/// An example strcut that contains a `Foo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Adt {
    x: u32,
    p: Ordered<Foo>,
}
