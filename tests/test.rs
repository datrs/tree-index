#![feature(iterator_step_by)]

extern crate tree_index as tree;
use tree::{Change, TreeIndex};

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
