use proc_macro::TokenStream;
use quote::quote;

use crate::utils::handle_macro::handle_macro;

pub fn controller_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let (ident, types, keys) = handle_macro(input.clone());

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
            all_routes: Vec<(
                    String,
                    String,
                    Box<
                        dyn Fn(rustle_core::RustleRequest, rustle_core::RustleResponse) -> rustle_core::RustleResponse
                            + Send
                            + Sync,
                    >,
                )>,
            #(#fields),*
        }

        impl #ident {
            pub fn new() -> Self {
                #ident {
                    all_routes: vec![],
                    #(#keys: rustle_core::RustleProvider.provide()),*
                }
            }
        }

        impl rustle_core::RustleController for #ident {
            fn name(&self) -> &str {
                stringify!(#ident)
            }

            fn add_route(
                &mut self,
                path: String,
                http_method: String,
                handler: Box<
                    dyn Fn(rustle_core::RustleRequest, rustle_core::RustleResponse) -> rustle_core::RustleResponse
                        + Send
                        + Sync,
                >,
            ) {
                self.all_routes.push((path, http_method, handler));
            }

            fn routes(&self) -> Vec<(
                String,
                String,
                &Box<
                    dyn Fn(rustle_core::RustleRequest, rustle_core::RustleResponse) -> rustle_core::RustleResponse
                        + Send
                        + Sync,
                >,
            )> {
                self.all_routes.iter().map(|(path, http_method, handler)| {
                    (path.clone(), http_method.clone(), handler.clone())
                }).collect()
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
