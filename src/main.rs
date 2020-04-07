use cfgstruct_macros::init;

#[derive(Debug)]
struct LateResources {
    #[cfg(feature = "feature_x")]
    x: u8,
    dummy: (),
}

#[init]
fn main() {
    // The cfg inside res disappears as seen from within proc_macro_attribute
    let res = LateResources {
        #[cfg(feature = "feature_x")]
        x: 0,
        dummy: ()
    };

    println!("{:#?}", res);

    // This cfg remains for the proc_macro_attribute to work on
    #[cfg(feature = "feature_x")]
    println!("Hello, world!");
}
