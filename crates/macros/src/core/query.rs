use proc_macro::TokenStream;
use quote::quote;
use syn::Field;

pub(crate) fn query_macro(input: TokenStream) -> TokenStream {
    let syn::ItemStruct {
        ident,
        generics,
        fields,
        ..
    } = syn::parse_macro_input!(input as syn::ItemStruct);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let fields: Vec<_> = fields
        .iter()
        .map(|Field { ident, .. }| {
            quote! {
                #ident: query.get(stringify!(#ident)).unwrap_or_default(),
            }
        })
        .collect();

    let expanded = quote! {
        impl #impl_generics ngyn::shared::server::Transformer<'_> for #ident #ty_generics #where_clause {
            fn transform(cx: &mut ngyn::prelude::NgynContext<'_>) -> Self {
                let query = ngyn::shared::server::Query::transform(cx);
                #ident {
                    #(#fields)*
                }
            }
        }
    };

    expanded.into()
}
