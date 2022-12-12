# MYH [mi:] serialization format
The myh serialization format aims to be self-contained and easy to use.
It throws as little errors as possible by design and sacrifices serde-comfort for
fast compile times and giving the developer full control of how to handle malformed
or missing values, which should result in no runtime surprises.

This library uses `nightly` because that's where all the fun features are anyways.

✨ IT IS NON PROC MACRO RELIANT ✨

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