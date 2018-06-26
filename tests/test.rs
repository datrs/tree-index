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
  assert_eq!(index.digest(0), 0b0, "has nothing");

  index = TreeIndex::default();
  index.set(0);
  assert_eq!(index.digest(0), 0b1, "has all");

  index = TreeIndex::default();
  index.set(1);
  assert_eq!(index.digest(0), 0b101, "rooted, no sibling, no parent");

  index = TreeIndex::default();
  index.set(2);
  assert_eq!(index.digest(0), 0b10, "not rooted, has sibling");

  index = TreeIndex::default();
  index.set(1);
  index.set(2);
  assert_eq!(index.digest(0), 0b1, "has all");

  index = TreeIndex::default();
  index.set(3);
  index.set(2);
  let left = index.digest(0);
  let right = 0b1011;
  assert_eq!(left, right, "rooted, sibling, no uncle, grand parents");

  index = TreeIndex::default();
  index.set(5);
  assert_eq!(index.digest(1), 0b10, "not rooted, has sibling");
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
fn proof_without_a_digest_1() {
  let mut index = TreeIndex::default();

  {
    let mut nodes = vec![];
    let proof = index.proof(0, &mut nodes, &mut TreeIndex::default());
    assert_eq!(proof, None);
  }

  {
    index.set(0);
    let mut nodes = vec![];
    let proof = index
      .proof(0, &mut nodes, &mut TreeIndex::default())
      .unwrap();
    assert_eq!(proof.nodes(), vec![].as_slice());
    assert_eq!(proof.verified_by(), 2);
  }

  {
    index.set(2);
    let mut nodes = vec![];
    let proof = index
      .proof(0, &mut nodes, &mut TreeIndex::default())
      .unwrap();
    assert_eq!(proof.nodes(), vec![2].as_slice());
    assert_eq!(proof.verified_by(), 4);
  }

  {
    index.set(5);
    let mut nodes = vec![];
    let proof = index
      .proof(0, &mut nodes, &mut TreeIndex::default())
      .unwrap();
    assert_eq!(proof.nodes(), vec![2, 5].as_slice());
    assert_eq!(proof.verified_by(), 8);
  }

  {
    index.set(8);
    let mut nodes = vec![];
    let proof = index
      .proof(0, &mut nodes, &mut TreeIndex::default())
      .unwrap();
    assert_eq!(proof.nodes(), vec![2, 5, 8].as_slice());
    assert_eq!(proof.verified_by(), 10);
  }
}

#[test]
fn proof_without_a_digest_2() {
  let mut index = TreeIndex::default();
  index.set(10);
  index.set(8);
  index.set(13);
  index.set(3);
  index.set(17);
  let mut nodes = vec![];
  let proof = index
    .proof(10, &mut nodes, &mut TreeIndex::default())
    .unwrap();
  assert_eq!(proof.nodes(), vec![8, 13, 3, 17].as_slice());
  assert_eq!(proof.verified_by(), 20);
}

#[test]
fn proof_without_a_digest_3() {
  let mut index = TreeIndex::default();
  index.set(7);
  index.set(16);
  index.set(18);
  index.set(21);
  index.set(25);
  index.set(28);

  {
    let mut nodes = vec![];
    let proof = index
      .proof(16, &mut nodes, &mut TreeIndex::default())
      .unwrap();
    assert_eq!(proof.nodes(), vec![18, 21, 7, 25, 28].as_slice());
    assert_eq!(proof.verified_by(), 30);
  }
  {
    let mut nodes = vec![];
    let proof = index
      .proof(18, &mut nodes, &mut TreeIndex::default())
      .unwrap();
    assert_eq!(proof.nodes(), vec![16, 21, 7, 25, 28].as_slice());
    assert_eq!(proof.verified_by(), 30);
  }
  {
    let mut nodes = vec![];
    let proof = index
      .proof(17, &mut nodes, &mut TreeIndex::default())
      .unwrap();
    assert_eq!(proof.nodes(), vec![21, 7, 25, 28].as_slice());
    assert_eq!(proof.verified_by(), 30);
  }
}

#[test]
fn proof_with_a_digest_1() {
  let mut index = TreeIndex::default();
  {
    let mut nodes = vec![];
    let proof = index.proof(0, &mut nodes, &mut TreeIndex::default());
    assert!(proof.is_none());
  }

  index.set(0);
  index.set(2);

  {
    let mut nodes = vec![];
    let proof = index
      .proof_with_digest(0, 1, &mut nodes, &mut TreeIndex::default())
      .unwrap();
    assert_eq!(proof.nodes(), vec![].as_slice());
    assert_eq!(proof.verified_by(), 0);
  }

  index.set(5);

  {
    let mut nodes = vec![];
    let proof = index
      .proof_with_digest(0, 0b10, &mut nodes, &mut TreeIndex::default())
      .unwrap();
    assert_eq!(proof.nodes(), vec![].as_slice());
    assert_eq!(proof.verified_by(), 0);
  }

  {
    let mut nodes = vec![];
    let proof = index
      .proof_with_digest(0, 0b110, &mut nodes, &mut TreeIndex::default())
      .unwrap();
    assert_eq!(proof.nodes(), vec![].as_slice());
    assert_eq!(proof.verified_by(), 0);
  }

  index.set(8);

  {
    let mut nodes = vec![];
    let proof = index
      .proof_with_digest(0, 0b101, &mut nodes, &mut TreeIndex::default())
      .unwrap();
    assert_eq!(proof.nodes(), vec![].as_slice());
    assert_eq!(proof.verified_by(), 0);
  }
  {
    let mut nodes = vec![];
    let proof = index
      .proof_with_digest(0, 0b10, &mut nodes, &mut TreeIndex::default())
      .unwrap();
    assert_eq!(proof.nodes(), vec![].as_slice());
    assert_eq!(proof.verified_by(), 0);
  }
}

// Test things don't crash.
#[test]
fn digest_sanity_checks() {
  let mut tree = TreeIndex::default();
  tree.set(0);
  let index = 0;
  let mut nodes = vec![];
  let mut remote_tree = TreeIndex::default();
  let digest = 999_999_999_999_999;
  tree.proof_with_digest(index, digest, &mut nodes, &mut remote_tree);
}
