use proc_macro::TokenStream;
use quote::quote;

pub(crate) fn dto_macro(input: TokenStream) -> TokenStream {
    let syn::ItemStruct {
        ident, generics, ..
    } = syn::parse_macro_input!(input as syn::ItemStruct);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ngyn::shared::server::Transformer<'_> for #ident #ty_generics #where_clause {
            fn transform(cx: &mut ngyn::prelude::NgynContext<'_>) -> Self {
                ngyn::prelude::Body::transform(cx).json::<#ident>().unwrap()
            }
        }

        impl #impl_generics ngyn::shared::server::ToBytes for #ident #ty_generics #where_clause {
            fn to_bytes(self) -> ngyn::shared::server::Bytes {
                ngyn::shared::server::Bytes::from(serde_json::to_string(&self).unwrap())
            }
        }
    };
    expanded.into()
}
