#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]
#![feature(external_doc)]
#![doc(include = "../README.md")]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

// https://github.com/mafintosh/hypercore/blob/master/lib/tree-index.js

extern crate flat_tree as flat;
extern crate sparse_bitfield as bitfield;

mod proof;
mod verification;

pub use self::bitfield::{Bitfield, Change};
pub use self::proof::Proof;
pub use self::verification::Verification;

/// Index a tree structure or something.
pub struct TreeIndex {
  bitfield: Bitfield,
}

impl TreeIndex {
  /// Create a new TreeIndex by passing it a sparse_bitfield instance.
  pub fn new(bitfield: Bitfield) -> Self {
    TreeIndex { bitfield }
  }

  /// Get a bit from the bitfield.
  pub fn get(&mut self, index: usize) -> bool {
    self.bitfield.get(index)
  }

  /// Set an index on the tree to `true`, and also all of the parents to the
  /// index. Walks the tree upward.
  ///
  /// Returns a "Change" member to indicate if the underlying value was changed.
  ///
  /// NOTE: we can probably change the bitfield.set syntax to return false to
  /// simplify this code a little.
  pub fn set(&mut self, index: usize) -> Change {
    if let Change::Unchanged = self.bitfield.set(index, true) {
      return Change::Unchanged;
    }

    let mut index = index;
    while self.bitfield.get(flat::sibling(index)) {
      index = flat::parent(index);
      if let Change::Unchanged = self.bitfield.set(index, true) {
        break;
      }
    }
    Change::Changed
  }

  /// Determine which Nodes prove the correctness for the Node at `index`.
  // - opts.hash: always push index to nodes.
  // - nodes: proven nodes.
  // - opts.digest: not sure what digest does.
  pub fn proof(&mut self, index: usize, nodes: Vec<usize>) -> Option<Proof> {
    if !self.get(index) {
      return None;
    }

    let mut digest = shift_right(index);
    let has_root = digest & 1;
    let mut sibling = index;
    let mut next = index;
    let mut roots = Vec::new(); // `null` in JavaScript

    while digest > 0 {
      if digest == 1 && has_root > 0 {
        if self.get(next) {
          // remote_tree.set(next); TODO
        }

        let tmp = flat::sibling(next);
        if tmp > next {
          next = tmp
        }

        flat::full_roots(flat::right_span(next) + 2, &mut roots);

        for root in roots {
          if self.get(root) {
            // remote-tree.set(root)
          }
        }
        break;
      }

      sibling = flat::sibling(next);
      if is_even(digest) {
        if self.get(sibling) {
          // remote-tree.set(sibling)
        }
      }

      next = flat::parent(next);
      digest = shift_right(digest);
    }

    Some(Proof {
      nodes: nodes,
      verified_by: 0,
    })
  }

  /// Create a digest for data at index.
  pub fn digest(&self) {
    unimplemented!();
  }

  /// Get the position of the highest entry in the tree. Aka max.
  ///
  /// NOTE: should we rename this to `.len()` ?
  pub fn blocks(&mut self) -> usize {
    let mut top = 0;
    let mut next = 0;
    let max = self.bitfield.len();

    while flat::right_span(next) < max {
      next = flat::parent(next);
      if self.get(next) {
        top = next;
      }
    }

    if self.get(top) {
      self.verified_by(top).node / 2
    } else {
      0
    }
  }

  /// Get all root nodes.
  ///
  /// TODO: don't make this allocate, but fill a vector instead.
  pub fn roots(&mut self, roots: &mut Vec<usize>) {
    flat::full_roots(2 * self.blocks(), roots)
  }

  /// Find the node that verified the node that's passed.
  ///
  /// This is different from the Javascript implementation in that it doesn't
  /// push the `top` value into an array, but returns it instead through the
  /// `Verification` type.
  pub fn verified_by(&mut self, index: usize) -> Verification {
    let has_index = self.get(index);
    if !has_index {
      return Verification {
        node: 0,
        top: 0,
      };
    }

    // Find root of current tree.
    let mut depth = flat::depth(index);
    let mut top = index;
    let mut parent = flat::parent_with_depth(top, depth);
    depth += 1;
    while self.get(parent) && self.get(flat::sibling(top)) {
      top = parent;
      parent = flat::parent_with_depth(top, depth);
      depth += 1;
    }

    // Expand right down.
    //
    // NOTE: this is probably a candidate to move to `flat-tree`.
    depth -= 1;
    while depth != 0 {
      top = flat::left_child_with_depth(
        flat::index(depth, flat::offset_with_depth(top, depth) + 1),
        depth,
      ).unwrap();
      depth -= 1;

      while !self.get(top) && depth > 0 {
        top = flat::left_child_with_depth(top, depth).unwrap();
        depth -= 1;
      }
    }

    let node = if self.get(top) {
      top + 2
    } else {
      top
    };

    Verification { node, top }
  }
}

/// Create a TreeIndex with an empty sparse_bitfield instance with a page size
/// of `1024`.
impl Default for TreeIndex {
  fn default() -> Self {
    TreeIndex {
      bitfield: Bitfield::new(1024),
    }
  }
}

// /// Do stuff with full roots.
// fn add_full_roots() {
//   unimplemented!();
// }

#[inline]
/// Check if a value is even.
fn is_even(n: usize) -> bool {
  match n & 1 {
    0 => true,
    1 => false,
    _ => panic!("Bitshift failure"),
  }
}

#[inline]
/// Bitwise shift numbers one to the right. e.g. 1001 (9) becomes 0100 (4).
fn shift_right(n: usize) -> usize {
  (n - (n & 1)) / 2
}

#[test]
fn shifts_bits_right() {
  assert_eq!(shift_right(9), 4);
  assert_eq!(shift_right(12), 6);
  assert_eq!(shift_right(13), 6);
  assert_eq!(shift_right(14), 7);
}
