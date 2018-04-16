#![feature(iterator_step_by)]

extern crate tree_index as tree;
use tree::{Change, TreeIndex, Verification};

#[test]
fn set_and_get() {
  let mut index = TreeIndex::default();

  assert_eq!(index.get(0), false);
  assert_eq!(index.set(0), Change::Changed);
  assert_eq!(index.get(0), true);
  assert_eq!(index.set(0), Change::Unchanged);

  assert_eq!(index.set(2), Change::Changed);
  assert_eq!(index.get(2), true);

  assert_eq!(index.get(1), true, "parent of 0 and 2 is set");

  let mut index = TreeIndex::default();
  // NOTE(yw): `.step_by()` is unstable.
  for i in (0..32).step_by(2) {
    index.set(i);
  }
  assert_eq!(index.get(7), true);
  assert_eq!(index.get(23), true);
  assert_eq!(index.get(15), true);
}

#[test]
fn digest() {
  let mut index = TreeIndex::default();
  assert_eq!(index.digest(0), num("0"), "has nothing");

  let mut index = TreeIndex::default();
  index.set(0);
  assert_eq!(index.digest(0), num("1"), "has all");

  let mut index = TreeIndex::default();
  index.set(1);
  assert_eq!(
    index.digest(0),
    num("101"),
    "rooted, no sibling, no parent"
  );

  let mut index = TreeIndex::default();
  index.set(2);
  assert_eq!(
    index.digest(0),
    num("10"),
    "not rooted, has sibling"
  );

  let mut index = TreeIndex::default();
  index.set(1);
  index.set(2);
  assert_eq!(index.digest(0), num("1"), "has all");

  let mut index = TreeIndex::default();
  index.set(3);
  index.set(2);
  assert_eq!(
    index.digest(0),
    num("1011"),
    "rooted, has sibling, no uncle, has grand parents"
  );

  let mut index = TreeIndex::default();
  index.set(5);
  assert_eq!(
    index.digest(1),
    num("10"),
    "not rooted, has sibling"
  );
}

#[test]
fn verified_by() {
  let mut index = TreeIndex::default();
  assert_eq!(
    index.verified_by(0),
    Verification {
      node: 0,
      top: 0,
    }
  );

  index.set(0);
  assert_eq!(
    index.verified_by(0),
    Verification {
      node: 2,
      top: 0,
    }
  );

  index.set(2);
  assert_eq!(
    index.verified_by(0),
    Verification {
      node: 4,
      top: 4,
    }
  );

  index.set(5);
  assert_eq!(
    index.verified_by(0),
    Verification {
      node: 8,
      top: 8,
    }
  );

  index.set(8);
  assert_eq!(
    index.verified_by(0),
    Verification {
      node: 10,
      top: 8,
    }
  );
}

fn num(input: &str) -> usize {
  usize::from_str_radix(input, 2).unwrap()
}
