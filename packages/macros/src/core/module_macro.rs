extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::utils::handle_macro::handle_macro;

pub fn module_macro(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (ident, types, keys) = handle_macro(input);

    let default_fields = vec![
        quote! { controllers: Vec<Box<dyn std::any::Any>> },
        quote! { providers: Vec<Box<dyn std::any::Any>> },
    ];

    let fields: Vec<_> = keys
        .iter()
        .zip(types.iter())
        .map(|(key, ty)| quote! { #key: #ty })
        .chain(
            default_fields
                .iter()
                .filter(|default| {
                    !keys
                        .iter()
                        .any(|key| format!("{:?}", default).contains(&key.to_string()))
                })
                .cloned(),
        )
        .collect();

    let expanded = quote! {
        use nject::injectable;
        use rustle_core::{RustleInjectable, RustleModule};

        #[injectable]
        pub struct #ident {
            #(#fields),*
        }

        impl RustleModule for #ident {
            /// Creates a new `#ident` with the specified components.
            ///
            /// # Examples
            ///
            /// ```
            /// let module = #ident::new();
            /// ```
            fn new() -> Self {
                #ident {
                    controllers: vec![],
                    providers: vec![],
                    // #(#keys: RustleInjectable::new()),*
                }
            }
        }
    };
    TokenStream::from(expanded)
}
