# cfgstructs

Why is the cfg-attribute missing in the proc_macro input?

Cargo build:

```rust
Input to proc_macro_attribute:
#####################################################
fn main() {
    let x = LateResources{x: 0, dummy: (),};
    println!("{:#?}", x);

    #[cfg(feature = "feature_x")]
    println!("Hello, world!");
}
#####################################################
End of proc macro input.
Why is the inner struct attribute missing?
```

Expected output from cargo build:

```rust
Input to proc_macro_attribute:
#####################################################
fn main() {
    let x = LateResources{#[cfg(feature = "feature_x")] x: 0, dummy: (),};
    println!("{:#?}", x);

    #[cfg(feature = "feature_x")]
    println!("Hello, world!");
}
#####################################################
End of proc macro input.
Why is the inner struct attribute missing?
```

Cargo run:

```rust
LateResources {
    dummy: (),
    }
```

Similar references:
[](https://stackoverflow.com/questions/49506485/how-to-provide-attributes-for-fields-for-struct-annotated-with-an-attribute-itse)
[](https://github.com/rust-lang/rust/issues/45358)
