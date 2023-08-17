extern crate proc_macro;

mod core;

use crate::core::module::module_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn module(attrs: TokenStream, input: TokenStream) -> TokenStream {
    module_macro(attrs, input)
}
