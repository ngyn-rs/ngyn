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

    let args = inputs.iter().filter_map(|input| {
        if let syn::FnArg::Typed(pat) = input {
            let (path, and_token, mutability) = match pat.ty.as_ref() {
                syn::Type::Reference(path_ref) => match path_ref.elem.as_ref() {
                    syn::Type::Path(path) => (&path.path, Some(path_ref.and_token), path_ref.mutability),
                    _ => panic!("Expected a reference or a path"),
                },
                syn::Type::Path(path) => (&path.path, None, None),
                _ => panic!("Expected a reference or a path"),
            };
            Some(quote! {
                #and_token #mutability ngyn::prelude::Transducer::reduce::<#and_token #mutability #path>(cx)
            })
        } else {
            None
        }
    }).reduce(|acc, arg| quote! { #acc, #arg });

    let gate_handlers = gates.iter().map(|path| {
        quote! {
            if !#path::can_activate(cx).await {
                return;
            }
        }
    });

    let middlewares_stream = middlewares.iter().map(|path| {
        quote! {
            #path::handle(cx).await;
        }
    });

    let exe_block = quote! {
        use ngyn::prelude::{NgynMiddleware, NgynGate};
        #(#middlewares_stream)*
        #(#gate_handlers)*
    };

    let body = match (asyncness.is_some(), &output) {
        (true, syn::ReturnType::Type(_, ty)) => quote! {
            let fn_body = (|#inputs| #asyncness move #block);
            Box::pin(#asyncness move {
                #exe_block;
                let output: #ty = fn_body(#args).await;
                *cx.response_mut().body_mut() = output.to_bytes().into();
            })
        },
        (true, syn::ReturnType::Default) => quote! {
            let fn_body = (|#inputs| #asyncness move #block);
            Box::pin(#asyncness move {
                #exe_block;
                fn_body(#args).await;
            })
        },
        (false, syn::ReturnType::Default) => quote! { (|#inputs| #block)(#args); },
        (false, syn::ReturnType::Type(_, ty)) => quote! {
            let output: #ty = (|#inputs| #block)(#args);
            *cx.response_mut().body_mut() = output.to_bytes().into();
        },
    };

    let output = asyncness.map(|_| {
        let r_arrow = RArrow::default();
        quote! { #r_arrow std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + '_cx_lifetime>> }
    });

    quote! {
        #vis #constness #unsafety #fn_token #ident <#generics_stream>(cx: &'_cx_lifetime mut ngyn::prelude::NgynContext) #output {
            #body
        }
    }
    .into()
}
