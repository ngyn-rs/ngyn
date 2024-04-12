use proc_macro::TokenStream;
use quote::quote;

pub fn dto_macro(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let ident = &input.ident;

    let expanded = quote! {
        impl ngyn::prelude::Transformer for #ident {
            fn transform(cx: &mut ngyn::prelude::NgynContext, res: &mut ngyn::prelude::NgynResponse) -> Self {
                let dto = ngyn::prelude::Dto::transform(cx, res);
                dto.parse::<#ident>().unwrap()
            }
        }
    };
    expanded.into()
}
