#![recursion_limit = "256"]
#![cfg_attr(can_show_location_of_runtime_parse_error, feature(proc_macro_span))]

extern crate proc_macro;
extern crate proc_macro2;
extern crate proc_macro_hack;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;

#[proc_macro_hack]
pub fn ruby(input: TokenStream) -> TokenStream {
    println!("{}", input.to_string());
    (quote! {
        println!("LOL");
    }).into()
}
