# MYH [myː] serialization format
The myh serialization format aims to be self-contained and easy to use.
It throws as little errors as possible by design and sacrifices serde-comfort for
fast compile times and giving the developer full control of how to handle malformed
or missing values, which should result in no runtime surprises.

This library uses `nightly` because that's where all the fun features are anyways.

✨ IT IS NON PROC MACRO RELIANT ✨

(we pretend that this is not similar to yaml)

## Example
```py
a_bool: true
b_i32: 42
c_sub: "Data 1"
    sub_string: "ABCDEF"
    sub_tuple: 75, 'c'
    inline_vec: 'x', 'y', 'z'
    outline_vec:
        - "a good reason to put this on multiple lines"
        - "yk this is rather long"
        - "and really really really bothersome to read in one line"
    - "a"
    - "toplevel"
    - "string"
d_vec:
    - 1
    - 2.7
    - 3.1415927
    - -4
```

## How to use
To get started take a look at the example above in 
[examples/structs](./examples/structs.rs) or check out any of the other [examples](./examples).

A myh element consists of three parts:
- an `item` (a primitive directly after the key)
- a `list` (a list of structs, denoted by the dashes)
- a `map` (a dictionary of structs with `key: value` syntax)

A primitive is a "single value", aka i32, u8, Range, char, String, etc, or a tuple of those.

Derive your type from `Primitive` if it only consists of a single value (ranges, ints, chars, etc...).
For structs and other composed types derive `Myh`.

### [comparison with toml](./test_files/serde_cargo.toml.myh)
(snippets from [serde toml](https://github.com/serde-rs/serde/blob/master/serde/Cargo.toml))
##### TOML
```toml
[package]
name = "serde"
version = "1.0.150" # remember to update html_root_url and serde_derive dependency
authors = ["Erick Tryzelaar <erick.tryzelaar@gmail.com>", "David Tolnay <dtolnay@gmail.com>"]
build = "build.rs"
categories = ["encoding", "no-std"]
/.../ # left out items for visibility

[dependencies]
serde_derive = { version = "=1.0.150", optional = true, path = "../serde_derive" }

[dev-dependencies]
serde_derive = { version = "1.0", path = "../serde_derive" }

[package.metadata.playground]
features = ["derive", "rc"]

[package.metadata.docs.rs]
targets = ["x86_64-unknown-linux-gnu"]
```
##### MYH
```py
package: "serde"
    version: "1.0.150" // remember to update html_root_url and serde_derive dependency
    authors:
        - "Erick Tryzelaar <erick.tryzelaar@gmail.com>"
        - "David Tolnay <dtolnay@gmail.com>"
    build: "build.rs"
    categories: "encoding", "no-std"
    /.../ // left out items for visibility
    metadata:
        playground:
            features: "derive", "rc"
        docs:
            rs:
                targets: "x86_64-unknown-linux-gnu"

dependencies:
    serde_derive: "=1.0.150"
        optional: true
        path: "../serde_derive"

dev_dependencies:
    serde_derive: "1.0"
        path: "../serde_derive"
```
##### MYH [_compacting keys with only one item_]
```py
package: "serde"
    /.../ // left out items for visibility
    metadata:
        playground: features: "derive", "rc"
        docs: rs: targets: "x86_64-unknown-linux-gnu"
```