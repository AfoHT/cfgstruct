# cfgstructs

Why is the cfg-attribute missing in the proc_macro input?

Cargo build, printout from within the macro, just printing and returning the `TokenStream`:

```rust
Input to proc_macro_attribute:
#####################################################
fn main() {
    let res = LateResources{x: 0, dummy: (),};
    println!("{:#?}", res);

    #[cfg(feature = "feature_x")]
    println!("Hello, world!");
}
#####################################################
End of proc macro input.
```

Expected output from cargo build:

```rust
Input to proc_macro_attribute:
#####################################################
fn main() {
    let res = LateResources{
        #[cfg(feature = "feature_x")]
        x: 0,
        dummy: (),
    };
    println!("{:#?}", res);

    #[cfg(feature = "feature_x")]
    println!("Hello, world!");
}
#####################################################
End of proc macro input.
```

However, both cargo run and cargo expand fully removes `x`.

Cargo run:

```rust
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/cfgstruct`
LateResources {
    dummy: (),
}
```
Cargo expand:

```rust
Input to proc_macro_attribute:
#####################################################
fn main() {
    let res =
        LateResources{
                      #[cfg(feature = "feature_x")]
                      x: 0,
                      dummy: (),};
    println!("{:#?}", res);

    #[cfg(feature = "feature_x")]
    println!("Hello, world!");
}
#####################################################
End of proc macro input.
    Finished check [unoptimized + debuginfo] target(s) in 5.24s

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use cfgstruct_macros::init;
struct LateResources {
    dummy: (),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for LateResources {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            LateResources {
                dummy: ref __self_0_0,
            } => {
                let mut debug_trait_builder = f.debug_struct("LateResources");
                let _ = debug_trait_builder.field("dummy", &&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
fn main() {
    let res = LateResources { dummy: () };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
            &["", "\n"],
            &match (&res,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
            &[::core::fmt::rt::v1::Argument {
                position: 0usize,
                format: ::core::fmt::rt::v1::FormatSpec {
                    fill: ' ',
                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                    flags: 4u32,
                    precision: ::core::fmt::rt::v1::Count::Implied,
                    width: ::core::fmt::rt::v1::Count::Implied,
                },
            }],
        ));
    };
}
```

Here the proper cfg stays inside `LateResources`.

Similar references:
[](https://stackoverflow.com/questions/49506485/how-to-provide-attributes-for-fields-for-struct-annotated-with-an-attribute-itse)
[](https://github.com/rust-lang/rust/issues/45358)
