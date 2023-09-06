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
        #[ngyn_core::dependency]
        #[derive(Clone)]
        pub struct #ident {
            #(#fields),*
        }

        impl #ident {
            pub fn new() -> Self {
                #ident {
                    #(#keys: ngyn_core::NgynProvider.provide()),*
                }
            }
        }

        impl ngyn_core::NgynInjectable for #ident {
            fn name(&self) -> &str {
                stringify!(#ident)
            }
        }

        impl ngyn_core::NgynInjectableInit for #ident {
            fn new() -> Box<dyn ngyn_core::NgynInjectable> {
                Box::new(Self::new())
            }
        }
    };
    expanded.into()
}
