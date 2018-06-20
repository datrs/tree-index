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
  for i in (0..32).step_by(2) {
    index.set(i);
  }
  assert_eq!(index.get(7), true);
  assert_eq!(index.get(23), true);
  assert_eq!(index.get(15), true);
}

#[test]
fn digest() {
  let mut index;
  index = TreeIndex::default();
  assert_eq!(index.digest(0), num("0"), "has nothing");

  index = TreeIndex::default();
  index.set(0);
  assert_eq!(index.digest(0), num("1"), "has all");

  index = TreeIndex::default();
  index.set(1);
  assert_eq!(index.digest(0), num("101"), "rooted, no sibling, no parent");

  index = TreeIndex::default();
  index.set(2);
  assert_eq!(index.digest(0), num("10"), "not rooted, has sibling");

  index = TreeIndex::default();
  index.set(1);
  index.set(2);
  assert_eq!(index.digest(0), num("1"), "has all");

  index = TreeIndex::default();
  index.set(3);
  index.set(2);
  let left = index.digest(0);
  let right = num("1011");
  assert_eq!(left, right, "rooted, sibling, no uncle, grand parents");

  index = TreeIndex::default();
  index.set(5);
  assert_eq!(index.digest(1), num("10"), "not rooted, has sibling");

  fn num(input: &str) -> usize {
    usize::from_str_radix(input, 2).unwrap()
  }
}

#[test]
fn verified_by() {
  let mut index = TreeIndex::default();

  verify(&mut index, 0, 0, 0);

  index.set(0);
  verify(&mut index, 0, 2, 0);

  index.set(2);
  verify(&mut index, 0, 4, 4);

  index.set(5);
  verify(&mut index, 0, 8, 8);

  index.set(8);
  verify(&mut index, 0, 10, 8);

  let mut index = TreeIndex::default();
  index.set(10);
  index.set(8);
  index.set(13);
  index.set(3);
  index.set(17);
  verify(&mut index, 10, 20, 20);

  let mut index = TreeIndex::default();
  index.set(7);
  index.set(16);
  index.set(18);
  index.set(21);
  index.set(25);
  index.set(28);
  verify(&mut index, 16, 30, 28);
  verify(&mut index, 18, 30, 28);
  verify(&mut index, 17, 30, 28);

  fn verify(tree: &mut TreeIndex, index: usize, node: usize, top: usize) {
    assert_eq!(tree.verified_by(index), Verification { node, top });
  }
}

#[test]
fn proof_without_a_digest() {
  let mut tree = TreeIndex::default();
  let index = 0;
  let mut nodes = vec![];
  let mut remote_tree = TreeIndex::default();
  let mut roots = vec![];
  assert_eq!(
    tree.proof(index, &mut nodes, &mut remote_tree, &mut roots),
    None
  );
}

#[test]
fn digest_sanity_checks() {
  let mut tree = TreeIndex::default();
  tree.set(0);
  let index = 0;
  let mut nodes = vec![];
  let mut remote_tree = TreeIndex::default();
  let mut roots = vec![];
  let digest = 999_999_999_999_999;
  tree.proof_with_digest(
    index,
    &mut nodes,
    &mut remote_tree,
    &mut roots,
    digest,
  ); // Did not crash
}
