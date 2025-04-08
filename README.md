# Surging Object Di-Graph, in Rust

[![EO principles respected here](https://www.elegantobjects.org/badge.svg)](https://www.elegantobjects.org)
[![We recommend IntelliJ IDEA](https://www.elegantobjects.org/intellij-idea.svg)](https://www.jetbrains.com/idea/)

[![cargo](https://github.com/objectionary/sodg/actions/workflows/cargo.yml/badge.svg)](https://github.com/objectionary/sodg/actions/workflows/cargo.yml)
[![crates.io](https://img.shields.io/crates/v/sodg.svg)](https://crates.io/crates/sodg)
[![PDD status](https://www.0pdd.com/svg?name=objectionary/sodg)](https://www.0pdd.com/p?name=objectionary/sodg)
[![codecov](https://codecov.io/gh/objectionary/sodg/branch/master/graph/badge.svg)](https://codecov.io/gh/objectionary/sodg)
[![Hits-of-Code](https://hitsofcode.com/github/objectionary/sodg)](https://hitsofcode.com/view/github/objectionary/sodg)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/objectionary/sodg/blob/master/LICENSE.txt)
[![docs.rs](https://img.shields.io/docsrs/sodg)](https://docs.rs/sodg/latest/sodg/)

This Rust library implements a Surging Object DiGraph (SODG) for
[reo](https://github.com/objectionary/reo) virtual machine for
[EO](https://www.eolang.org) programs. The graph is "surging" because
it automatically behind the scene deletes vertices and edges from itself,
which is also known as "garbage collection" mechanism. A vertex gets deleted
right after the data it contains is read _and_ no other vertices
transitively point to it.

Here is how you can create a di-graph:

```rust
use sodg::Sodg;
use sodg::Hex;
let mut g = Sodg::empty(256);
g.add(0); // add a vertex no.0
g.add(1); // add a vertex no.1
g.bind(0, 1, "foo"); // connect v0 to v1 with label "foo"
g.put(1, &Hex::from_str_bytes("Hello, world!")); // attach data to v1
```

Then, you can find a vertex by the label of an edge departing
from another vertex:

```rust
let id = g.kid(0, "foo");
assert_eq!(1, id);
```

Then, you can find all kids of a vertex:

```rust
let kids: Vec<(String, String, usize)> = g.kids(0);
assert_eq!("foo", kids[0].0);
assert_eq!("bar", kids[0].1);
assert_eq!(1, kids[0].2);
```

Then, you can read the data of a vertex:

```rust
let hex: Hex = g.data(1);
let num: i64 = hex.to_i64()?;
assert_eq!(42, num);
```

Then, you can print the graph:

```rust
println!("{:?}", g);
```

Using `merge()`, you can merge two graphs together, provided they are trees.

Using `save()` and `load()`, you can serialize and deserialize the graph.

Using `to_xml()` and `to_dot()`, you can print it to
[XML](https://en.wikipedia.org/wiki/XML) and
[DOT](https://graphviz.org/doc/info/lang.html).

Using `slice()` and `slice_some()`, you can take a part/slice
of the graph (mostly for debugging purposes).

Read [the documentation](https://docs.rs/sodg/latest/sodg/).

## How to Contribute

First, install [Rust](https://www.rust-lang.org/tools/install) and then:

```bash
cargo test -vv
```

If everything goes well, fork repository, make changes, send us a
[pull request](https://www.yegor256.com/2014/04/15/github-guidelines.html).
We will review your changes and apply them to the `master` branch shortly,
provided they don't violate our quality standards. To avoid frustration,
before sending us your pull request please run `cargo test` again. Also,
run `cargo fmt` and `cargo clippy`.
