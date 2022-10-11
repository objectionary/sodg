<img alt="logo" src="https://www.objectionary.com/cactus.svg" height="100px" />

[![EO principles respected here](https://www.elegantobjects.org/badge.svg)](https://www.elegantobjects.org)
[![We recommend IntelliJ IDEA](https://www.elegantobjects.org/intellij-idea.svg)](https://www.jetbrains.com/idea/)

[![cargo](https://github.com/objectionary/sodg/actions/workflows/cargo.yml/badge.svg)](https://github.com/objectionary/sodg/actions/workflows/cargo.yml)
[![crates.io](https://img.shields.io/crates/v/sodg.svg)](https://crates.io/crates/sodg)
[![PDD status](http://www.0pdd.com/svg?name=objectionary/sodg)](http://www.0pdd.com/p?name=objectionary/sodg)
[![codecov](https://codecov.io/gh/objectionary/sodg/branch/master/graph/badge.svg)](https://codecov.io/gh/objectionary/sodg)
[![Hits-of-Code](https://hitsofcode.com/github/objectionary/sodg)](https://hitsofcode.com/view/github/objectionary/sodg)
![Lines of code](https://img.shields.io/tokei/lines/github/objectionary/sodg)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/objectionary/sodg/blob/master/LICENSE.txt)

This Rust library helps you build a simple object tree (SODG) for
[reo](https://github.com/objectionary/reo) compiler of
[EO](https://www.eolang.org) programs.

Create a tree:

```rust
use sodg::Sodg;
let mut sodg = Sodg::empty();
sodg.add(0)?; // add a vertex no.0
sodg.add(1)?; // add a vertex no.1
sodg.bind(0, 1, "foo")?; // connect v0 to v1 with label "foo"
sodg.put(1, "Hello, world!".as_bytes().to_vec())?; // attach data to v1
```

You can find a vertex by the label of an edge departing from another vertex:

```rust
let id = sodg.kid(0, "foo")?; // returns 1
```

You can find all kids of a vertex:

```rust
let kids: Vec<(String, u32)> = sodg.kids(0);
```

You can read the data of a vertex:

```rust
let bytes: Vec<u8> = sodg.data(1)?; // empty if no data written before
```

Then, you can print the tree:

```rust
println!("{:?}", sodg);
```

Also, you can serialize and deserialize the tree.
