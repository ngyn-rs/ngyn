use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, Signature};

use super::routes::RouteArgs;

pub(crate) fn route_macro(args: TokenStream, raw_input: TokenStream) -> TokenStream {
    let input = raw_input.clone();
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = syn::parse_macro_input!(input as ItemFn);

    let Signature {
        ident,
        inputs,
        output,
        generics,
        constness,
        asyncness,
        unsafety,
        ..
    } = sig;

    if let Some(syn::FnArg::Receiver(_)) = inputs.first() {
        // Skip transforming methods, only transform functions
        // We handle methods in the `routes` macro
        raw_input
    } else {
        let RouteArgs { http_method, path } = syn::parse_macro_input!(args as RouteArgs);

        let mut routes = Vec::new();
        if let Some(path) = path {
            path.each(|path| {
                routes.push(quote! {(
                    #http_method.to_string(),
                    #path.to_string(),
                    stringify!(#ident).to_string(),
                )});
            });
        }

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
        let handle_body = if asyncness.is_some() {
            match output {
                syn::ReturnType::Type(_, _) => quote! {
                    let body = self.#ident(#args).await;
                    res.send(body);
                },
                _ => quote! {
                    self.#ident(#args).await;
                },
            }
        } else {
            match output {
                syn::ReturnType::Type(_, _) => quote! {
                    let body = self.#ident(#args);
                    res.send(body);
                },
                _ => quote! {
                    self.#ident(#args);
                },
            }
        };

        let expanded = quote::quote! {
            #(#attrs)*
            #vis #constness #asyncness #unsafety fn #ident #generics(#inputs) #output #block

            impl #generics ngyn::shared::traits::NgynInjectable for #ident {
                fn new() -> Self {
                    Self
                }
            }

            #[ngyn::prelude::async_trait]
            impl #generics ngyn::shared::traits::NgynController for #ident {
                fn routes(&self) -> Vec<(String, String, String)> {
                    vec![#(#routes),*]
                }

                async fn handle(
                    &mut self,
                    handler: &str,
                    cx: &mut ngyn::prelude::NgynContext,
                    res: &mut ngyn::prelude::NgynResponse,
                ) {
                    self.inject(cx);
                    if handler == stringify!(#ident) {
                        #handle_body;
                    }
                }
            }
        };
        expanded.into()
    }
}
