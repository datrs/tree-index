#![feature(iterator_step_by)]

extern crate tree_index as tree;
use tree::{Change, Proof, TreeIndex, Verification};

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
}

#[test]
fn proof_without_a_digest() {
  let mut index = TreeIndex::default();
  assert_eq!(index.proof(0), None);
}

fn num(input: &str) -> usize {
  usize::from_str_radix(input, 2).unwrap()
}

// Shorthand function to verify a tree-index and some values.
fn verify(tree: &mut TreeIndex, index: usize, node: usize, top: usize) {
  assert_eq!(
    tree.verified_by(index),
    Verification { node, top }
  );
}

// Shorthand function to prove a proof.
fn prove(
  tree: &mut TreeIndex,
  index: usize,
  nodes: Vec<usize>,
  verified_by: usize,
) {
  let proof = Proof::new(verified_by, nodes);
  assert_eq!(tree.proof(index), Some(proof));
}
