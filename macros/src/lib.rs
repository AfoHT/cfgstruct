// src/lib.rs (default entry point for proc macros)

extern crate proc_macro;  // Apparently needed to be imported like this.

use proc_macro::TokenStream;

#[proc_macro_attribute]   // Can now be used as `#[my_attribute]`
pub fn init(_attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("Input to proc_macro_attribute:\n#####################################################\n{}", item.to_string());
    println!("#####################################################\nEnd of proc macro input.\nWhy is the inner struct attribute missing?");
    item
}
