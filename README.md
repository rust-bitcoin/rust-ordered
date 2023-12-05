Arbitrary Ordering
==================

Provides a wrapper for types that can technically implement `PartialOrd`/`Ord`
but for semantic reasons it is nonsensical.

## Examples

You might want to use a type as a key in a `BTreeMap`, this requires `Ord` even though it might be
nonsensical to implement a total order for the type.

```rust
use std::cmp::Ordering;
use std::collections::BTreeMap;
use ordered::{ArbitraryOrd, Ordered};

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

let a = Foo::Space(50);
let b = Foo::Time(50);

let mut map = BTreeMap::new();

// error[E0277]: the trait bound `Foo: Ord` is not satisfied
// map.insert(a, "some interesting value");

map.insert(Ordered(a), "some interesting value");
map.insert(Ordered(b), "some other interesting value");
```

Perhaps you would like to derive `Ord` on a complex type that contains a type that does not
implement `Ord`.

```rust
/// An example strcut that contains a `Foo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Adt {
    x: u32,
    p: Ordered<Foo>,
}

// Then there are various ways to get at the inner data.

let adt = Adt { x: 42, p: Foo::Space(50).into() };

println!("We can explicitly deref: {}", *adt.p);
println!("Or use deref coercion: {}", adt.p);
println!("Or we can use borrow: {}", &adt.p);

// And if all that is too complicated just use the inherent methods:

println!("Explicitly get a reference: {}", adt.p.as_inner());
println!("Or the inner type: {}", adt.p.into_inner());
```

## Minimum Supported Rust Version (MSRV)

The crate MSRV is Rust v1.56.1

## Licensing

The code in this project is licensed under the [Creative Commons CC0 1.0 Universal license](LICENSE).
We use the [SPDX license list](https://spdx.org/licenses/) and [SPDX IDs](https://spdx.dev/ids/).
