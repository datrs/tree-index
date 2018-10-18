# tree-index
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Stateful tree index. Or well, stateful flat-tree. It's what happens when you
combine a flat-tree with a sparse-bitfield - which ends up being pretty cool!

Adapted from
[mafintosh/hypercore/lib/tree-index.js](https://github.com/mafintosh/hypercore/blob/master/lib/tree-index.js).

- [Documentation][8]
- [Crates.io][2]

## Usage
```rust
extern crate sparse_bitfield as bitfield;
extern crate tree_index;

use tree_index::TreeIndex;
use self::bitfield::{Bitfield, Change};

let bitfield = Bitfield::new(1024);
let mut tree = TreeIndex::new(bitfield);
assert_eq!(tree.set(0), Change::Changed);
assert_eq!(tree.set(0), Change::Unchanged);
assert_eq!(tree.get(0), true);
assert_eq!(tree.get(1), false);
```

## Installation
```sh
$ cargo add tree-index
```

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/tree-index.svg?style=flat-square
[2]: https://crates.io/crates/tree-index
[3]: https://img.shields.io/travis/datrs/tree-index/master.svg?style=flat-square
[4]: https://travis-ci.org/datrs/tree-index
[5]: https://img.shields.io/crates/d/tree-index.svg?style=flat-square
[6]: https://crates.io/crates/tree-index
[7]: https://docs.rs/tree-index/badge.svg
[8]: https://docs.rs/tree-index
