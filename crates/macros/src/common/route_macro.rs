use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, Signature};

pub fn route_macro(_args: TokenStream, raw_input: TokenStream) -> TokenStream {
    let ItemFn {
        sig,
        block,
        vis,
        attrs,
    } = match syn::parse::<ItemFn>(raw_input) {
        Ok(input) => input,
        Err(err) => {
            let error_message = err.to_string();
            panic!("failed to parse method: {}", error_message)
        }
    };

    let Signature {
        ident,
        inputs,
        output,
        fn_token,
        ..
    } = sig;

    let return_val = match output {
        syn::ReturnType::Type(_, _) => quote! {},
        _ => quote! { return ngyn::prelude::NgynBody::None; },
    };

    let output = match output {
        syn::ReturnType::Type(_, ty) => quote! { -> #ty }, // TODO: Handle other types aside NgynBody
        _ => quote! { -> ngyn::prelude::NgynBody },
    };

    let expanded = quote! {
        #(#attrs)*
        #vis async #fn_token #ident(#inputs) #output {
            #block
            #return_val
        }
    };

    expanded.into()
}
