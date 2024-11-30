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
        if gates.is_empty() {
            return Err(syn::Error::new(input.span(), "expected at least one gate"));
        }
        Ok(HandlerArgs {
            gates: vec![],
            middlewares: vec![],
        })
    }
}

pub fn handler_macro(args: TokenStream, raw_input: TokenStream) -> TokenStream {
    let HandlerArgs { gates, .. } = syn::parse::<HandlerArgs>(args).unwrap();
    let ItemFn { sig, .. } =
        syn::parse::<ItemFn>(raw_input).expect("Only impl blocks are supported");
    let Signature {
        ident,
        inputs,
        asyncness,
        output,
        ..
    } = sig.clone();
    let ident_str = ident.to_string();

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
                let body = self.#ident(#args).await;
                *res.body_mut() = body.to_bytes().into();
            },
            _ => quote! {
                self.#ident(#args).await;
            },
        }
    } else {
        match output {
            syn::ReturnType::Type(_, _) => quote! {
                let body = self.#ident(#args);
                *res.body_mut() = body.to_bytes().into();
            },
            _ => quote! {
                self.#ident(#args);
            },
        }
    };
    quote! {
        #ident_str => {
            #gate_handlers
            #handle_body
        }
    }
    .into()
}
