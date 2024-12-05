use proc_macro::TokenStream;
use quote::quote;

pub(crate) fn service_macro(input: TokenStream) -> TokenStream {
    let syn::ItemStruct {
        ident,
        generics,
        fields,
        ..
    } = syn::parse_macro_input!(input as syn::ItemStruct);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields: Vec<_> = fields
        .iter()
        .map(
            |syn::Field {
                 ident, colon_token, ..
             }| {
                quote! {
                    #ident #colon_token Default::default()
                }
            },
        )
        .collect();

    let expanded = quote! {
        impl #impl_generics ngyn::shared::server::Transformer<'_> for #ident #ty_generics #where_clause {
            fn transform(cx: &mut ngyn::prelude::NgynContext) -> Self {
                Self::default()
            }
        }

        impl #impl_generics Default for #ident #ty_generics #where_clause {
            fn default() -> Self {
                Self {
                    #(#fields)*
                }
            }
        }
    };
    expanded.into()
}
