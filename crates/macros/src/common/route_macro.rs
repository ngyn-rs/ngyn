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
        asyncness,
        generics,
        ..
    } = sig;

    let return_val = match output {
        syn::ReturnType::Type(_, _) => quote! {},
        _ => quote! { return ngyn::prelude::Bytes::default(); },
    };

    let output = match output {
        syn::ReturnType::Type(_, ty) => quote! { -> #ty },
        _ => quote! { -> ngyn::prelude::Bytes },
    };

    let expanded = quote! {
        #(#attrs)*
        #vis #asyncness #fn_token #ident #generics (#inputs) #output {
            #block
            #return_val
        }
    };

    expanded.into()
}
