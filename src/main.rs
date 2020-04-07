use cfgstruct_macros::init;

#[derive(Debug)]
struct LateResources {
    #[cfg(feature = "feature_x")]
    x: u8,
    dummy: (),
}

#[init]
fn main() {
    let x = LateResources {
        #[cfg(feature = "feature_x")]
        x: 0,
        dummy: ()
    };


    // cargo run produces:
    /*
      LateResources {
          dummy: (),
          }
     */
    println!("{:#?}", x);

    // This works
    #[cfg(feature = "feature_x")]
    println!("Hello, world!");
}
