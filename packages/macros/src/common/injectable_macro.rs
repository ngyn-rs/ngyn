use proc_macro::TokenStream;
use quote::quote;

use crate::utils::handle_macro::handle_macro;

pub fn injectable_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let (ident, types, keys) = handle_macro(input);

    let fields: Vec<_> = types
        .iter()
        .zip(keys.iter())
        .map(|(ty, key)| quote! { #key: #ty })
        .collect();

    let expanded = quote! {
        #[rustle_core::dependency]
        pub struct #ident {
            #(#fields),*
        }

        impl #ident {
            pub fn create_new() -> Self {
                #ident {
                    #(#keys: rustle_core::RustleProvider.provide()),*
                }
            }
        }

        impl rustle_core::RustleInjectable for #ident {}

        impl rustle_core::RustleInjectableInit for #ident {
            fn new() -> Box<dyn rustle_core::RustleInjectable> {
                Box::new(Self::create_new())
            }
        }
    };
    expanded.into()
}
