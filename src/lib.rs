// Copyright 2018 Phillip Lord, Newcastle University
//
// Licensed under either the Apache License, Version 2.0 or the MIT
// licence at your option. This file may not be copied, modified or
// distributed except according to those terms.

/*!
This crate enables attaching metadata to C-like Enums (or strictly any
Enum). The metadata can be of an arbitrary type, but must be of the
same type for the all variants although can be different values.

This fills the use-case when the Enum variants are flags for something
else -- for example, HTTP error codes, or parts of a syntax tree
associated with some explicit string rendering when concretized.

The crate provides two macros which can be used to add this metadata
to the enum. This can be done at a separate location from the
declaration of the enum. The first macro is for values that are
determined at compile time:

# Syntax
```ignore
meta! {
  EnumType, MetaDataType;
  VariantOne, "MetadataValue";
  VariantTwo, "MetadataValue";
  VariantThree, "MetadataValue";
}
```

In this case, the type of the metadata must be defined before hand and
will be either a reference type or a copy type, as the values will be
returned statically. For example:

```rust
#[macro_use] extern crate enum_meta;
use enum_meta::*;
enum Colour
{
   Red, Orange, Green
}

meta!{
   Colour, &'static str;
   Red, "Red";
   Orange, "Orange";
   Green, "Green";
}

fn main() {
   assert_eq!(Colour::Orange.meta(), "Orange");
}
```

A second macro allows the values to be calculated at run time on first
access. The values are calculated only once.

```rust
#[macro_use] extern crate enum_meta;
use enum_meta::*;
pub enum Colour{
    Red,
    Orange,
    Green
}

lazy_meta!{
    Colour, String, META_Colour;
    Red, format!("{}:{}", 1, "Red");
    Orange, format!("{}:{}", 2, "Orange");
    Green, format!("{}:{}", 3, "Green");
}

fn main() {
   assert_eq!(Colour::Red.meta(), "1:Red");
}
```

In this case, values are stored in a global variable whose name is
provided (`META_Colour2` in this instance). Values returned are
references to the given return type.

Reverse lookup is not supported in-directly, by providing an `all`
method which returns all the enum variants as a vector; this allows
construction of a reverse lookup function; this is hard to achieve in
general, requires putting a lot of constraints on the type of the
metadata and can only sensibly support lookup by direct equality with
the metadata.

```
#[macro_use] extern crate enum_meta;
use enum_meta::*;

// These derives are required by `assert_eq` rather than `lazy_meta`
#[derive(Debug, Eq, PartialEq)]
pub enum Colour{
    Red,
    Orange,
    Green
}

meta!{
    Colour, String;
    Red, format!("{}:{}", 1, "Red");
    Orange, format!("{}:{}", 2, "Orange");
    Green, format!("{}:{}", 3, "Green");
}

fn main() {
    assert_eq!(Colour::all(),
              vec![Colour::Red, Colour::Orange, Colour::Green]);
}
```


*/
#![macro_use]

#[allow(unused_imports)]
#[macro_use] extern crate lazy_static;


pub use lazy_static::*;

pub use std::collections::HashMap;
pub use std::mem::discriminant;
pub use std::mem::Discriminant;

/// Trait for accessing metadata
pub trait Meta<R>
    where Self:Sized {
    fn meta(&self) -> R;
    fn all() -> Vec<Self>;
}

#[macro_export]
macro_rules! meta {
    ($enum_type:ident, $return_type:ty;
     $($enum_variant:ident, $return_value:expr);*
    ) => {
        impl Meta<$return_type> for $enum_type {

            fn meta(&self) -> $return_type {
                match self {
                    $(
                        $enum_type::$enum_variant => {
                            $return_value
                        }
                    )*
                }
            }

            fn all() -> Vec<$enum_type>{
                vec![
                    $(
                        $enum_type::$enum_variant
                    ),*
                ]
            }
        }
    };
    // Trailing semi
    ($enum_type:ident, $return_type:ty;
     $($enum_variant:ident, $return_value:expr);+ ;
    ) => {
        meta!{
            $enum_type, $return_type;
            $( $enum_variant, $return_value );*
        }
    };
}

#[macro_export]
macro_rules! lazy_meta {
    ($enum_type:ident, $return_type:ty, $storage:ident;
     $($enum_variant:ident, $return_expr:expr);*
    ) => {
        lazy_static! {
            static ref $storage: HashMap<Discriminant<$enum_type>,$return_type>
                = {
                    let mut m = HashMap::new();

                    $(
                        m.insert(discriminant(&$enum_type::$enum_variant),
                                 $return_expr);
                    )*
                        m
                };
        }

        impl <'a> Meta<&'a $return_type> for $enum_type {
            fn meta(&self) -> &'a $return_type {
                $storage.get(&discriminant(&self)).unwrap()
            }

            fn all() -> Vec<$enum_type>{
                vec![
                    $(
                        $enum_type::$enum_variant
                    ),*
                ]
            }
        }

        impl $enum_type {
            // This does nothing at all, but will fail if we do not pass all of
            // the entities that we need.
            #[allow(dead_code)]
            fn meta_check(&self) {
                match self {
                    $(
                        $enum_type::$enum_variant => {}
                    ),*
                }
            }
        }
    };
    // Trailing semi
    ($enum_type:ident, $return_type:ty, $storage:ident;
     $($enum_variant:ident, $return_expr:expr);+ ;
    ) => {
        lazy_meta!{
            $enum_type, $return_type, $storage;
            $( $enum_variant, $return_expr );*
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_meta(){
        enum Colour
        {
            Red,
            Orange,
            Green
        }

        meta!{
            Colour, &'static str;
            Red, "Red";
            Orange, "Orange";
            Green, "Green"
        }

        assert_eq!(Colour::Red.meta(), "Red");
        assert_eq!(Colour::Orange.meta(), "Orange");
        assert_eq!(Colour::Green.meta(), "Green");
    }

    #[test]
    fn test_all(){
        #[derive(Debug, Eq, PartialEq)]
        enum Colour
        {
            Red,
            Orange,
            Green
        }

        meta!{
            Colour, &'static str;
            Red, "Red";
            Orange, "Orange";
            Green, "Green"
        }

        assert_eq!(vec![Colour::Red,
                        Colour::Orange,
                        Colour::Green], Colour::all());
    }

    #[test]
    fn test_meta_complex_return_type(){
        enum Colour
        {
            Red,
            Orange,
            Green
        }

        meta!{
            Colour, (&'static str, i64);
            Red, ("Red", 10);
            Orange, ("Orange", 11);
            Green, ("Green", 12)
        }

        assert_eq!(Colour::Red.meta(), ("Red", 10));
        assert_eq!(Colour::Orange.meta(), ("Orange", 11));
        assert_eq!(Colour::Green.meta(), ("Green", 12));
    }

    #[test]
    fn test_meta_trailing_semi(){
        enum Colour
        {
            Red,
            Orange,
            Green
        }

        meta!{
            Colour, &'static str;
            Red, "Red";
            Orange, "Orange";
            Green, "Green";
        }

        assert_eq!(Colour::Red.meta(), "Red");
        assert_eq!(Colour::Orange.meta(), "Orange");
        assert_eq!(Colour::Green.meta(), "Green");
    }

    #[test]
    fn test_lazy_meta(){
        enum Colour
        {
            Red,
            Orange,
            Green
        }

        lazy_meta!{
            Colour, String, TEST1;
            Red, "Red".to_string();
            Orange, "Orange".to_string();
            Green, "Green".to_string();
        }

        assert_eq!(Colour::Red.meta(), "Red");
        assert_eq!(Colour::Orange.meta(), "Orange");
        assert_eq!(Colour::Green.meta(), "Green");
    }

    #[test]
    fn test_lazy_all(){
        #[derive(Debug, Eq, PartialEq)]
        enum Colour
        {
            Red,
            Orange,
            Green
        }

        lazy_meta!{
            Colour, String, TEST1;
            Red, "Red".to_string();
            Orange, "Orange".to_string();
            Green, "Green".to_string();
        }

        assert_eq!(Colour::all(),
                   vec![Colour::Red,
                        Colour::Orange,
                        Colour::Green]
                   );
    }
}
