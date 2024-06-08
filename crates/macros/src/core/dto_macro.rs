use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn dto_macro(input: TokenStream) -> TokenStream {
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
    };
    expanded.into()
}
