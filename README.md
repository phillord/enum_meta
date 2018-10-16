enum-meta
=========

This crate enables attaching metadata to C-like Enums (or strictly any
Enum). The metadata can be of an arbitrary type, but must be of the
same type for the all variants although can be different values.


[![Travis-CI Status](https://travis-ci.org/phillord/enum_meta.svg?branch=master)](https://travis-ci.org/phillord/enum_meta)

This fills the use-case when the Enum variants are flags for something
else -- for example, HTTP error codes, or parts of a syntax tree
associated with some explicit string rendering when concretized.

The crate provides two macros which can be used to add this metadata
to the enum. This can be done at a separate location from the
declaration of the enum. The first macro is for values that are
determined at compile time:

```rust
enum Colour {
   Red, Orange, Green
}

meta!{
   Colour, &'static str;
   Red, "Red";
   Orange, "Orange";
   Green, "Green";
}

assert_eq!(Red.meta(), "Red");
```

And the second for where values are calculated at runtime.

```rust
pub enum Colour{
    Red,
    Orange,
    Green
}

lazy_meta!{
    Colour, &String, META_Colour2
    Red, format!("{}:{}", 1, "Red");
    Orange, format!("{}:{}", 2, "Orange");
    Green, format!("{}:{}", 3, "Green");
}
```

The former returns values directly, while the second returns
references. Values are only calculated once on first usage.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
