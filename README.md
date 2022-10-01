This Rust library helps you build an object tree for
reo compiler of EO programs.

Create a tree:

```rust
use cot::Tree;
let tree = Tree::new();
tree.add(0); // add a vertex no.1
tree.add(1); // add a vertex no.1
tree.bind(0, 1, "foo"); // connect v0 to v1 with label "foo"
tree.data(1, "Hello, world!".as_bytes()); // attachd data to v1
```

You can find a vertex by the label of an edge departing from another vertex:

```rust
let id = tree.kid(0, "foo"); // returns 1
```

You can find all kids of a vertex:

```rust
let kids: Vec<(u32, String)> = tree.kids(0);
```

Then, you can print the tree:

```rust
println!("{:?}", tree);
```

Also, you can serialize and deserialize the tree.
