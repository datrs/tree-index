## 2020-07-09, Version 0.6.1
### Commits
- [[`b559443333`](https://github.com/datrs/tree-index/commit/b559443333adf4c1da34eb68507eebe8fcd4510c)] (cargo-release) version 0.6.1 (Bruno Tavares)
- [[`1935f708bc`](https://github.com/datrs/tree-index/commit/1935f708bc815654e6cd45173940b8187d9d644b)] Merge pull request #18 from khodzha/as_bitfield (Bruno Tavares)
- [[`edd6bf54d4`](https://github.com/datrs/tree-index/commit/edd6bf54d451d6116fd97d6e65d84ac3154e9fa3)] added as_bitfield() -> &Bitfield method (Shamir Khodzha)
- [[`0eb38f3b97`](https://github.com/datrs/tree-index/commit/0eb38f3b972e677f71da4f087aa2e3b521af42ca)] Update changelog (Bruno Tavares)

### Stats
```diff
 CHANGELOG.md | 21 +++++++++++++++++++++
 Cargo.toml   |  2 +-
 src/lib.rs   |  5 +++++
 3 files changed, 27 insertions(+), 1 deletion(-)
```


## 2020-03-03, Version 0.6.0
### Commits
- [[`9e7d303f35`](https://github.com/datrs/tree-index/commit/9e7d303f3598debb3b96940d4d787830c9abcfd0)] (cargo-release) version 0.6.0 (Bruno Tavares)
- [[`fab4ef7730`](https://github.com/datrs/tree-index/commit/fab4ef7730863e69ffdf03c049adcfbe2d6c1cb2)] Merge pull request #17 from bltavares/usize-to-u64 (Bruno Tavares)
- [[`0134d747e4`](https://github.com/datrs/tree-index/commit/0134d747e46d9755d46d2fd009ed1dcd641c9ec1)] Point flat-tree to crates version (Bruno Tavares)
- [[`7213b1dfcd`](https://github.com/datrs/tree-index/commit/7213b1dfcd67679048b365a929fb890d28ded1e3)] Change from usize to u64 (Bruno Tavares)
- [[`1198d8a7ca`](https://github.com/datrs/tree-index/commit/1198d8a7ca2b59351eda8b48bded8bf1bf666ddd)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 .travis.yml         |  6 +++---
 CHANGELOG.md        | 22 ++++++++++++++++++++++-
 Cargo.toml          |  6 +++---
 src/lib.rs          | 55 ++++++++++++++++++++++++------------------------------
 src/proof.rs        | 14 +++++++-------
 src/verification.rs |  4 ++--
 tests/test.rs       | 20 +++++++++++++-------
 7 files changed, 75 insertions(+), 52 deletions(-)
```


## 2018-10-18, Version 0.5.0
### Commits
- [[`1e6b8eeb50`](https://github.com/datrs/tree-index/commit/1e6b8eeb50bbf7b405788cf4377ddac556bb7059)] (cargo-release) version 0.5.0 (Yoshua Wuyts)
- [[`a8a4cc6f9b`](https://github.com/datrs/tree-index/commit/a8a4cc6f9bbd9f633de1a352cee5d23c885c2dd1)] enable to work on stable (#11) (Yoshua Wuyts)
- [[`babd4a450c`](https://github.com/datrs/tree-index/commit/babd4a450c86c8ea43aea5553ba087d0e19c8bb3)] Update sparse-bitfield requirement from 0.7.0 to 0.8.1 (#10) (dependabot[bot])
- [[`5fd7abe492`](https://github.com/datrs/tree-index/commit/5fd7abe492444429816eda8bb254a31cf22b7653)]  Keep up with modern times in clippy invocation (#9) (Szabolcs Berecz)
- [[`7ff59dfd42`](https://github.com/datrs/tree-index/commit/7ff59dfd420915632940d2de6f676d6648a90275)] Merge pull request #8 from ZhouHansen/comment-blocks (周汉成)
- [[`1c1ff326ba`](https://github.com/datrs/tree-index/commit/1c1ff326baaea6fa768cc904a3ecdf8a07655197)] add comment (ZhouHansen)
- [[`c6ad5124d7`](https://github.com/datrs/tree-index/commit/c6ad5124d7726c07b260339b7ec179eb61ab34ef)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 .travis.yml   | 24 +++++++++++++-----------
 CHANGELOG.md  | 21 +++++++++++++++++++++
 Cargo.toml    |  6 +++---
 README.md     |  4 ++--
 src/lib.rs    | 48 +++++++++++++++++++++++++++++++++++++++++++-----
 tests/test.rs | 20 ++++++--------------
 6 files changed, 88 insertions(+), 35 deletions(-)
```


## 2018-09-03, Version 0.4.1
### Commits
- [[`271ed7d7db`](https://github.com/datrs/tree-index/commit/271ed7d7db23c32e9f82a253bb3fc9a598567575)] (cargo-release) version 0.4.1 (Yoshua Wuyts)
- [[`80977f5fac`](https://github.com/datrs/tree-index/commit/80977f5face6c68fb5754ea286ca26002440aee2)] use bitwise operator (#5) (周汉成)
- [[`766bb7dd82`](https://github.com/datrs/tree-index/commit/766bb7dd82e7ea0b4227a5d0b3d031886c47e09a)] update .github (Yoshua Wuyts)
- [[`82e189ef4e`](https://github.com/datrs/tree-index/commit/82e189ef4eb6b4dba4f9003e946f3601eeecb149)] Switch to clippy-preview (#3) (Szabolcs Berecz)
- [[`f3d0e3833b`](https://github.com/datrs/tree-index/commit/f3d0e3833ba7a457f0c9183f8a63b0dad0a28a4a)] (cargo-release) start next development iteration 0.4.1-alpha.0 (Yoshua Wuyts)

### Stats
```diff
 .github/ISSUE_TEMPLATE.md                 | 40 +++-----------------------------
 .github/ISSUE_TEMPLATE/bug_report.md      | 23 ++++++++++++++++++-
 .github/ISSUE_TEMPLATE/feature_request.md | 30 ++++++++++++++++++++++++-
 .github/ISSUE_TEMPLATE/question.md        | 18 ++++++++++++++-
 .travis.yml                               |  6 +++--
 Cargo.toml                                |  2 +-
 src/lib.rs                                | 21 ++---------------
 7 files changed, 83 insertions(+), 57 deletions(-)
```


