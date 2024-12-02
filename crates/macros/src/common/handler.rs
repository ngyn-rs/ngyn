use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{token::RArrow, ItemFn, Signature};

pub(super) struct HandlerArgs {
    gates: Vec<syn::Path>,
    middlewares: Vec<syn::Path>,
}

impl syn::parse::Parse for HandlerArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut gates = vec![];
        let mut middlewares = vec![];

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
    generics_stream.extend(quote! { '_ctx_lifetime });

    let args = inputs.iter().fold(None, |args, input| {
        if let syn::FnArg::Typed(pat) = input {
            let ty = &pat.ty;
            let (path, and_token, mutability) = {
                if let syn::Type::Reference(path_ref) = *ty.clone() {
                    if let syn::Type::Path(path) = *path_ref.elem.clone() {
                        (
                            path.path,
                            Some(path_ref.and_token),
                            path_ref.mutability,
                        )
                    } else {
                        panic!("Expected a reference or a path");
                    }
                } else if let syn::Type::Path(path) = *ty.clone() {
                    (path.path, None, None)
                } else {
                    panic!("Expected a reference or a path");
                }
            };
            let arg_def = quote! {
                #and_token #mutability ngyn::prelude::Transducer::reduce::<#and_token #mutability #path>(cx)
            };
            if args.is_none() {
                Some(quote! {
                    #arg_def
                })
            } else {
                Some(quote! {
                    #args, #arg_def
                })
            }
        } else {
            args
        }
    });

    let gate_handlers = gates.iter().fold(quote! {}, |gates, path| {
        quote! {
            #gates
            if !#path::can_activate(cx).await {
                return;
            }
        }
    });

    let middlewares_stream = middlewares.iter().fold(quote! {}, |middlewares, path| {
        quote! {
            #middlewares
            #path::handle(cx).await;
        }
    });

    let exe_block = quote! {
        use ngyn::prelude::NgynMiddleware;
        use ngyn::prelude::NgynGate;
        #middlewares_stream
        #gate_handlers
    };

    let handle_body = if asyncness.is_some() {
        match output {
            syn::ReturnType::Type(_, _) => quote! {
                let body = (|#inputs| async move #block)(#args);
                Box::pin(async move {
                    #exe_block;
                    *cx.response().body_mut() = body.await.to_bytes().into();
                })
            },
            _ => quote! {
                let body = (|#inputs| async move #block)(#args);
                Box::pin(async move {
                    #exe_block
                    body.await;
                })
            },
        }
    } else {
        match output {
            syn::ReturnType::Type(_, _) => quote! {
                *cx.response().body_mut() = (|#inputs| #block)(#args).to_bytes().into();
            },
            _ => quote! {
                (|#inputs| #block)(#args)
            },
        }
    };
    let output = if asyncness.is_some() {
        let r_arrow = RArrow::default();
        Some(
            quote! { #r_arrow std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + '_ctx_lifetime>> },
        )
    } else {
        None
    };
    quote! {
        #vis #constness #unsafety #fn_token #ident <#generics_stream>(cx: &'_ctx_lifetime mut ngyn::prelude::NgynContext) #output {
            #handle_body
        }
    }
    .into()
}
