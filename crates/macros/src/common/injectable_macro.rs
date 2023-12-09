use proc_macro::TokenStream;
use quote::quote;

use crate::utils::parse_macro_data::parse_macro_data;

pub fn injectable_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
    let syn::DeriveInput {
        ident,
        data,
        attrs,
        vis,
        ..
    } = syn::parse_macro_input!(input as syn::DeriveInput);
    let (types, keys) = parse_macro_data(data);

    let fields: Vec<_> = types
        .iter()
        .zip(keys.iter())
        .map(|(ty, key)| quote! { #key: #ty })
        .collect();

    let expanded = quote! {
        #(#attrs)*
        #[ngyn::dependency]
        #[derive(Clone)]
        #vis struct #ident {
            #(#fields),*
        }

        impl ngyn::NgynInjectable for #ident {
            fn new() -> Self {
                #ident {
                    #(#keys: ngyn::NgynProvider.provide()),*
                }
            }
        }
    };
    expanded.into()
}
