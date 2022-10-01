<img alt="logo" src="https://www.objectionary.com/cactus.svg" height="100px" />

This Rust library helps you build an object tree for
[reo](https://github.com/objectionary/reo) compiler of
[EO](https://www.eolang.org) programs.

Create a tree:

```rust
use cot::Tree;
let tree = Tree::new();
tree.add(0); // add a vertex no.1
tree.add(1); // add a vertex no.1
tree.bind(0, 1, "foo"); // connect v0 to v1 with label "foo"
tree.write(1, "Hello, world!".as_bytes()); // attach data to v1
```

You can find a vertex by the label of an edge departing from another vertex:

```rust
let id = tree.kid(0, "foo"); // returns 1
```

You can find all kids of a vertex:

```rust
let kids: Vec<(u32, String)> = tree.kids(0);
```

You can read the data of a vertex:

```rust
let bytes: Vec<u8> = tree.read(1); // empty if no data written before
```

Then, you can print the tree:

```rust
println!("{:?}", tree);
```

Also, you can serialize and deserialize the tree.
