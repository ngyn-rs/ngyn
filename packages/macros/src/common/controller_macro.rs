use proc_macro::TokenStream;
use quote::quote;

use crate::utils::handle_macro::handle_macro;

pub fn controller_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let (ident, types, keys) = handle_macro(input);

    let fields: Vec<_> = types
        .iter()
        .zip(keys.iter())
        .map(|(ty, key)| quote! { #key: #ty })
        .collect();

    let expanded = quote! {
        use nject::injectable as nject_injectable;
        use rustle_core::{RustleController, RustleControllerInit, RustleInjectableInit, RustleProvider};

        #[nject_injectable]
        pub struct #ident {
            #(#fields),*
        }

        impl #ident {
            pub fn create_new() -> Self {
                #ident {
                    #(#keys: RustleProvider.provide()),*
                }
            }
        }

        impl RustleController for #ident {
            fn routes(&self) -> Vec<(&str, &str, Box<dyn Fn() + Send + Sync>)> {
                vec![]
            }
        }

        impl RustleControllerInit for #ident {
            fn new() -> Box<dyn RustleController> {
                Box::new(Self::create_new())
            }
        }
    };
    expanded.into()
}
