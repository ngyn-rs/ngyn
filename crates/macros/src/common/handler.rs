use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, Signature};

pub(super) struct HandlerArgs {
    gates: Vec<syn::Path>,
    middlewares: Vec<syn::Path>,
}

impl syn::parse::Parse for HandlerArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut gates = vec![];
        while !input.is_empty() {
            let path: syn::Path = input.parse()?;
            gates.push(path);
            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(HandlerArgs {
            gates,
            middlewares: vec![],
        })
    }
}

pub fn handler_macro(args: TokenStream, raw_input: TokenStream) -> TokenStream {
    let HandlerArgs { gates, .. } = syn::parse::<HandlerArgs>(args).unwrap();
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
        abi,
        generics,
        paren_token,
        variadic,
        ..
    } = sig.clone();

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
                #and_token #mutability ngyn::prelude::Transducer::reduce::<#and_token #mutability #path>(cx, res)
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
            {
                use ngyn::prelude::NgynGate;
                let mut gate = #path::default();
                gate.inject(cx);
                if !gate.can_activate(cx, res).await {
                    return;
                }
            }
        }
    });
    let handle_body = if asyncness.is_some() {
        match output {
            syn::ReturnType::Type(_, _) => quote! {
                Box::pin(async {
                    *res.body_mut() = async #block.await.to_bytes().into();
                })
            },
            _ => quote! {
                Box::pin(async {
                    #block;
                })
            },
        }
    } else {
        match output {
            syn::ReturnType::Type(_, _) => quote! {
                *res.body_mut() = #block.to_bytes().into();
            },
            _ => quote! {
                #block;
            },
        }
    };
    let output = if asyncness.is_some() {
        quote! { -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'b>> }
    } else {
        quote! {}
    };
    quote! {
        #vis #constness #unsafety #fn_token #ident <'a, 'b>(cx: &'a mut ngyn::prelude::NgynContext, res: &'b mut ngyn::prelude::NgynResponse) #output {
            #gate_handlers
            #handle_body
        }
    }
    .into()
}
