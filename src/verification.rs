/// Returned by `.verified_by()`.
pub struct Verification {
  /// Node that verifies the index.
  pub node: usize,
  /// The highest Node found.
  pub top: usize,
}
