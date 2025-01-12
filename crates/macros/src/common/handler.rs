use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{token::RArrow, ItemFn, Signature};

pub(super) struct HandlerArgs {
    gates: Vec<syn::Path>,
    middlewares: Vec<syn::Path>,
}

impl syn::parse::Parse for HandlerArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut gates = Vec::new();
        let mut middlewares = Vec::new();

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;

            let content;
            // parse a list of middleware paths, [middleware1, middleware2]
            syn::bracketed!(content in input);

            while !content.is_empty() {
                match ident.to_string().as_str() {
                    "gates" => {
                        gates.push(content.parse()?);
                    }
                    "middlewares" => {
                        middlewares.push(content.parse()?);
                    }
                    _ => {
                        return Err(syn::Error::new(
                            ident.span(),
                            format!("unexpected argument `{}`", ident),
                        ));
                    }
                }
                if !content.is_empty() {
                    content.parse::<syn::Token![,]>()?;
                }
            }

            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(HandlerArgs { gates, middlewares })
    }
}

pub fn handler_macro(args: TokenStream, raw_input: TokenStream) -> TokenStream {
    let HandlerArgs {
        gates, middlewares, ..
    } = syn::parse::<HandlerArgs>(args).unwrap();
    let ItemFn {
        sig, block, vis, ..
    } = syn::parse::<ItemFn>(raw_input).expect("Only functions blocks are supported");
    let Signature {
        ident,
        inputs,
        asyncness,
        output,
        fn_token,
        constness,
        unsafety,
        generics,
        ..
    } = sig.clone();

    if asyncness.is_none() && (!gates.is_empty() || !middlewares.is_empty()) {
        panic!("Gates and middlewares are only supported with async handlers");
    }

    let mut generics_stream = generics.to_token_stream();
    generics_stream.extend(quote! { '_cx_lifetime });

    let args = inputs
        .iter()
        .map(|input| {
            if let syn::FnArg::Typed(_) = input {
                quote! { ngyn::prelude::Transducer::reduce(cx) }
            } else {
                panic!("Only associated functions are supported");
            }
        })
        .reduce(|args, arg| quote! { #args, #arg });

    let gate_handlers = gates.iter().map(|path| {
        quote! {
            if !#path::can_activate(cx).await {
                return Box::new(()) as Box<dyn ngyn::prelude::ToBytes>;
            }
        }
    });

    let middlewares_stream = middlewares.iter().map(|path| {
        quote! {
            #path::handle(cx).await;
        }
    });

    let exe_block = quote! {
        use ngyn::prelude::{NgynMiddleware, NgynGate, ToBytes};
        #(#middlewares_stream)*
        #(#gate_handlers)*
    };

    let body = match asyncness.is_some() {
        true => quote! {
            async fn handle(#inputs) #output #block
            let body = handle(#args);
            Box::pin(#asyncness move {
                #exe_block;
                Box::new(body.await) as Box<dyn ngyn::prelude::ToBytes>
            })
        },
        false => quote! {
            let output = (|#inputs| #block)(#args);
            Box::new(output) as Box<dyn ngyn::prelude::ToBytes>
        },
    };

    let output = asyncness.map(|_| {
        let r_arrow = RArrow::default();
        quote! { #r_arrow std::pin::Pin<Box<dyn std::future::Future<Output = Box<dyn ngyn::prelude::ToBytes>> + Send + '_cx_lifetime>> }
    }).unwrap_or_else(|| quote! { -> Box<dyn ngyn::prelude::ToBytes> });

    quote! {
        #vis #constness #unsafety #fn_token #ident <#generics_stream>(cx: &'_cx_lifetime mut ngyn::prelude::NgynContext) #output {
            #body
        }
    }
    .into()
}
