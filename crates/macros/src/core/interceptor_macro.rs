use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

pub fn interceptor_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let name = &input.ident;

    let expanded = quote! {
        #[ngyn_core::dependency]
        impl #name {
            /// The `intercept` function takes a mutable reference to any type.
            /// It is intended to be overridden with logic to intercept and potentially alter the execution of a function.
            pub fn intercept(&self, execution: &mut dyn std::any::Any) {
                // TODO: Implement the logic of the interceptor here
            }
        }
    };

    TokenStream::from(expanded)
}
