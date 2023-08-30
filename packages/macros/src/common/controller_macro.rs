use proc_macro::TokenStream;
use quote::quote;

use crate::utils::handle_macro::handle_macro;

pub fn controller_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let (ident, types, keys) = handle_macro(input);

    let fields: Vec<_> = types
        .iter()
        .zip(keys.iter())
        .map(|(ty, key)| {
            quote! {
                #[allow(dead_code)]
                #key: #ty
            }
        })
        .collect();

    let expanded = quote! {
        #[rustle_core::dependency]
        pub struct #ident {
            #(#fields),*
        }

        impl #ident {
            pub fn new() -> Self {
                #ident {
                    #(#keys: rustle_core::RustleProvider.provide()),*
                }
            }
        }

        impl rustle_core::RustleController for #ident {
            fn routes(&self) -> Vec<(
                &str,
                &str,
                Box<
                    dyn Fn(rustle_core::RustleRequest, rustle_core::RustleResponse) -> rustle_core::RustleResponse
                        + Send
                        + Sync,
                >,
            )> {
                vec![]
            }
        }

        impl rustle_core::RustleControllerInit for #ident {
            fn new() -> Box<dyn rustle_core::RustleController> {
                Box::new(Self::new())
            }
        }
    };
    expanded.into()
}
