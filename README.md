<img alt="logo" src="https://www.objectionary.com/cactus.svg" height="100px" />

[![EO principles respected here](https://www.elegantobjects.org/badge.svg)](https://www.elegantobjects.org)
[![We recommend IntelliJ IDEA](https://www.elegantobjects.org/intellij-idea.svg)](https://www.jetbrains.com/idea/)

[![cargo](https://github.com/objectionary/sot/actions/workflows/cargo.yml/badge.svg)](https://github.com/objectionary/sot/actions/workflows/cargo.yml)
[![crates.io](https://img.shields.io/crates/v/sot.svg)](https://crates.io/crates/sot)
[![PDD status](http://www.0pdd.com/svg?name=objectionary/sot)](http://www.0pdd.com/p?name=objectionary/sot)
[![codecov](https://codecov.io/gh/objectionary/sot/branch/master/graph/badge.svg)](https://codecov.io/gh/objectionary/sot)
[![Hits-of-Code](https://hitsofcode.com/github/objectionary/sot)](https://hitsofcode.com/view/github/objectionary/sot)
![Lines of code](https://img.shields.io/tokei/lines/github/objectionary/sot)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/objectionary/sot/blob/master/LICENSE.txt)

This Rust library helps you build an object tree for
[reo](https://github.com/objectionary/reo) compiler of
[EO](https://www.eolang.org) programs.

Create a tree:

```rust
use sot::Sot;
let mut sot = Sot::empty();
sot.add(0)?; // add a vertex no.1
sot.add(1)?; // add a vertex no.1
sot.bind(0, 1, "foo")?; // connect v0 to v1 with label "foo"
sot.put(1, "Hello, world!".as_bytes().to_vec())?; // attach data to v1
```

You can find a vertex by the label of an edge departing from another vertex:

```rust
let id = sot.kid(0, "foo")?; // returns 1
```

You can find all kids of a vertex:

```rust
let kids: Vec<(u32, String)> = sot.kids(0);
```

You can read the data of a vertex:

```rust
let bytes: Vec<u8> = sot.data(1)?; // empty if no data written before
```

Then, you can print the tree:

```rust
println!("{:?}", sot);
```

Also, you can serialize and deserialize the tree.
