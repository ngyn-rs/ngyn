use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn dto_macro(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = syn::parse_macro_input!(input as DeriveInput);

    let expanded = quote! {
        impl #generics ngyn::prelude::Transformer<'_> for #ident #generics {
            fn transform(cx: &mut ngyn::prelude::NgynContext, res: &mut ngyn::prelude::NgynResponse) -> Self {
                let dto = ngyn::prelude::Dto::transform(cx, res);
                dto.parse::<#ident>().unwrap()
            }
        }

        impl #generics ngyn::shared::server::ToBytes for #ident #generics {
            fn to_bytes(self) -> ngyn::shared::server::Bytes {
                ngyn::shared::server::Bytes::from(serde_json::to_string(&self).unwrap())
            }
        }
    };
    expanded.into()
}
