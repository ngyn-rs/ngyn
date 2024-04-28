use proc_macro::TokenStream;
use quote::quote;

pub fn dto_macro(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let ident = &input.ident;

    let expanded = quote! {
        impl ngyn::prelude::Transformer for #ident {
            fn transform(cx: &mut ngyn::prelude::NgynContext, res: &mut ngyn::prelude::NgynResponse) -> Option<Self> {
                let dto = ngyn::prelude::Dto::transform(cx, res).unwrap();
                match dto.parse::<#ident>() {
                    Ok(data) => {
                        if let Err(err) = data.run_validation() {
                            res.set_status(400);
                            res.send(err.to_string());
                            return None;
                        }
                        return Some(data);
                    }
                    Err(err) => {
                        res.set_status(400);
                        res.send(err.to_string());
                        return None;
                    }
                }
            }
        }
    };
    expanded.into()
}
