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
        #[ngyn::dependency]
        #[derive(Clone)]
        pub struct #ident {
            #(#fields),*
        }

        impl #ident {
            pub fn new() -> Self {
                #ident {
                    #(#keys: ngyn::NgynProvider.provide()),*
                }
            }
        }

        impl ngyn::NgynInjectable for #ident {
            fn name(&self) -> &str {
                stringify!(#ident)
            }
        }

        impl ngyn::NgynInjectableInit for #ident {
            fn new() -> Box<dyn ngyn::NgynInjectable> {
                Box::new(Self::new())
            }
        }
    };
    expanded.into()
}
