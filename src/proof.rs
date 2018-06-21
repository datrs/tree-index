/// A merkle proof for an index.
///
/// Merkle trees are proven by checking the parent hashes.
#[derive(Debug, PartialEq)]
pub struct Proof<'a> {
  verified_by: usize,
  nodes: &'a [usize],
}

impl <'a>Proof<'a> {
  /// Create a new instance.
  ///
  /// ## Examples
  /// ```rust
  /// # extern crate tree_index;
  /// # use tree_index::Proof;
  /// let nodes = vec![];
  /// let _proof = Proof::new(0, &nodes);
  /// ```
  #[inline]
  pub fn new(verified_by: usize, nodes: &'a [usize]) -> Self {
    Self { nodes, verified_by }
  }

  /// Get the index for the node which verifies the input index.
  ///
  /// ## Examples
  /// ```rust
  /// # extern crate tree_index;
  /// # use tree_index::Proof;
  /// let nodes = vec![];
  /// let proof = Proof::new(0, &nodes);
  /// assert_eq!(proof.verified_by(), 0);
  /// ```
  #[inline]
  pub fn verified_by(&self) -> usize {
    self.verified_by
  }

  /// Merkle proof for the index you pass, written in `flat-tree` notation.
  ///
  /// ## Examples
  /// ```rust
  /// # extern crate tree_index;
  /// # use tree_index::Proof;
  /// let nodes = vec![];
  /// let proof = Proof::new(0, &nodes);
  /// assert_eq!(proof.nodes.to_owned(), vec![]);
  /// ```
  #[inline]
  pub fn nodes(&self) -> &[usize] {
    &self.nodes
  }
}
