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
            panic!("failed to parse route: {}", error_message)
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

    let output = match output {
        syn::ReturnType::Type(r, ty) => quote! { #r #ty },
        _ => quote! {},
    };

    let expanded = quote! {
        #(#attrs)*
        #vis #asyncness #fn_token #ident #generics (#inputs) #output {
            #block
        }
    };

    expanded.into()
}
