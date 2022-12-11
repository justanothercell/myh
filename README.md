# MYH [mi:] serialization format
The myh serialization format aims to be self-contained and easy to use.
It throws as little errors as possible by design and sacrifices serde-comfort for
fast compile times and giving the developer full control of how to handle malformed
or missing values, which should result in no runtime surprises.

This library uses `nightly` because that's where all the fun features are anyways.

IT IS NON PROC MACRO RELIANT!

## How to use
Derive your type from `Primitive` if it only consists of a single value.
For structs and other composed types derive `Myh`