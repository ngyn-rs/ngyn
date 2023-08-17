extern crate proc_macro;

mod core;

use crate::core::{interceptor::interceptor_macro, module::module_macro};
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn module(args: TokenStream, input: TokenStream) -> TokenStream {
    module_macro(args, input)
}

#[proc_macro_attribute]
pub fn interceptor(args: TokenStream, input: TokenStream) -> TokenStream {
    interceptor_macro(args, input)
}
