/// A merkle proof for an index.
///
/// Merkle trees are proven by checking the parent hashes.
#[derive(Debug, PartialEq)]
pub struct Proof<'a> {
  index: u64,
  verified_by: u64,
  nodes: &'a [u64],
}

impl<'a> Proof<'a> {
  /// Create a new instance.
  ///
  /// ## Examples
  /// ```rust
  /// # extern crate tree_index;
  /// # use tree_index::Proof;
  /// let nodes = vec![];
  /// let _proof = Proof::new(0, 0, &nodes);
  /// ```
  #[inline]
  pub fn new(index: u64, verified_by: u64, nodes: &'a [u64]) -> Self {
    Self {
      index,
      nodes,
      verified_by,
    }
  }

  /// Get the index which was used to verify this node.
  ///
  /// ## Examples
  /// ```rust
  /// # extern crate tree_index;
  /// # use tree_index::Proof;
  /// let nodes = vec![];
  /// let proof = Proof::new(0, 0, &nodes);
  /// assert_eq!(proof.index(), 0);
  /// ```
  #[inline]
  pub fn index(&self) -> u64 {
    self.index
  }

  /// Get the index for the node which verifies the input index.
  ///
  /// ## Examples
  /// ```rust
  /// # extern crate tree_index;
  /// # use tree_index::Proof;
  /// let nodes = vec![];
  /// let proof = Proof::new(0, 0, &nodes);
  /// assert_eq!(proof.verified_by(), 0);
  /// ```
  #[inline]
  pub fn verified_by(&self) -> u64 {
    self.verified_by
  }

  /// Merkle proof for the index you pass, written in `flat-tree` notation.
  ///
  /// ## Examples
  /// ```rust
  /// # extern crate tree_index;
  /// # use tree_index::Proof;
  /// let nodes = vec![];
  /// let proof = Proof::new(0, 0, &nodes);
  /// assert_eq!(proof.nodes().len(), 0);
  /// ```
  #[inline]
  pub fn nodes(&self) -> &[u64] {
    &self.nodes
  }
}
