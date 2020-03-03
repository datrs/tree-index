/// Returned by `.verified_by()`.
#[derive(Debug, PartialEq)]
pub struct Verification {
  /// Node that verifies the index.
  pub node: u64,
  /// The highest Node found.
  pub top: u64,
}
