use ngyn_shared::server::Method;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemImpl, Signature};

/// Path Enum
///
/// This enum represents the possible types of paths that can be used in the application.
/// It can either be a Single path represented as a String or Multiple paths represented as a Vector of Strings.
#[derive(Debug, Clone)]
pub(super) enum RoutePath {
    /// Represents a single path as a String
    Single(String),
    /// Represents multiple paths as a Vector of Strings
    Multiple(Vec<String>),
}

impl RoutePath {
    pub fn each<F>(&self, mut f: F)
    where
        F: FnMut(&str),
    {
        match self {
            RoutePath::Single(path) => f(path),
            RoutePath::Multiple(paths) => paths.iter().for_each(|path| f(path)),
        }
    }
}

pub(super) struct PathArg {
    pub path: Option<RoutePath>,
}

impl syn::parse::Parse for PathArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = if input.peek(syn::LitStr) {
            let path = input.parse::<syn::LitStr>()?;
            Some(RoutePath::Single(path.value()))
        } else if input.peek(syn::token::Bracket) {
            let content;
            syn::bracketed!(content in input);
            let mut paths = Vec::new();
            while !content.is_empty() {
                let path = content.parse::<syn::LitStr>()?;
                paths.push(path.value());
                if !content.is_empty() {
                    content.parse::<syn::Token![,]>()?;
                }
            }
            Some(RoutePath::Multiple(paths))
        } else {
            None
        };

        Ok(PathArg { path })
    }
}

pub(super) struct RouteArgs {
    pub path: Option<RoutePath>,
    pub http_method: String,
}

impl syn::parse::Parse for RouteArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let http_method = input.parse::<syn::LitStr>()?.value();

        if Method::from_bytes(http_method.as_bytes()).is_err() {
            panic!("Unsupported HTTP method: {}", http_method)
        } else {
            input.parse::<syn::Token![,]>()?;
        }

        let PathArg { path } = input.parse::<PathArg>()?;

        Ok(RouteArgs { path, http_method })
    }
}

struct CheckArgs {
    gates: Vec<syn::Path>,
}

impl syn::parse::Parse for CheckArgs {
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
        Ok(Self { gates })
    }
}

pub(crate) fn routes_macro(raw_input: TokenStream) -> TokenStream {
    let ItemImpl {
        items,
        attrs,
        defaultness,
        unsafety,
        impl_token,
        generics,
        trait_,
        self_ty,
        ..
    } = match syn::parse::<ItemImpl>(raw_input) {
        Ok(input) => input,
        Err(_err) => panic!("Only impl blocks are supported"),
    };

    if let Some((..)) = trait_ {
        panic!("Trait impls are not supported");
    }

    let mut route_defs: Vec<_> = Vec::new();
    let mut handle_routes: Vec<_> = Vec::new();

    for item in items.clone() {
        if let syn::ImplItem::Fn(method) = item {
            for attr in method.attrs.clone() {
                if attr.path().is_ident("route")
                    || attr.path().is_ident("get")
                    || attr.path().is_ident("post")
                    || attr.path().is_ident("put")
                    || attr.path().is_ident("delete")
                    || attr.path().is_ident("patch")
                    || attr.path().is_ident("head")
                    || attr.path().is_ident("options")
                {
                    let (path, http_method) = {
                        if attr.path().is_ident("route") {
                            let RouteArgs { path, http_method } =
                                attr.parse_args::<RouteArgs>().unwrap();
                            (path, http_method)
                        } else {
                            let PathArg { path } = attr.parse_args::<PathArg>().unwrap();
                            let http_method = attr.path().get_ident().unwrap().to_string();
                            (path, http_method)
                        }
                    };
                    path.unwrap().each(|path| {
                        let Signature {
                            ident,
                            inputs,
                            asyncness,
                            output,
                            ..
                        } = method.sig.clone();
                        let ident_str = ident.to_string();
                        route_defs.push(quote! {(
                            #path,
                            #http_method,
                            #ident_str,
                        )});
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
                        let http_code = method.attrs.iter().find_map(|attr| {
                            if attr.path().is_ident("http_code") {
                                Some(attr.meta.clone())
                            } else {
                                None
                            }
                        });
                        let http_code = match http_code {
                            Some(http_code) => quote! {
                                res.set_status(#http_code);
                            },
                            None => quote! {},
                        };
                        let gates = method.attrs.iter().filter_map(|attr| {
                            if attr.path().is_ident("check") {
                                Some(attr.meta.clone())
                            } else {
                                None
                            }
                        });
                        let gate_handlers: Vec<_> = gates
                            .map(|gate| {
                                if let syn::Meta::List(path) = gate {
                                    let CheckArgs { gates } =
                                        path.parse_args::<CheckArgs>().unwrap();
                                    gates.iter().fold(quote! {}, |gates, path| {
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
                                    })
                                } else {
                                    panic!("Expected a list of gates")
                                }
                            })
                            .collect();
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
                        handle_routes.push(quote! {
                            #ident_str => {
                                #http_code
                                #(#gate_handlers)*
                                #handle_body
                            }
                        });
                    })
                }
            }
        }
    }

    let expanded = quote! {
        #defaultness #unsafety #(#attrs)*
        #impl_token #generics #self_ty  {
            const ROUTES: &'static [(&'static str, &'static str, &'static str)] = &[
                #(#route_defs),*
            ];
            #(#items)*

            async fn __handle_route(
                &mut self,
                handler: &str,
                cx: &mut ngyn::prelude::NgynContext,
                res: &mut ngyn::prelude::NgynResponse
            ) {
                res.set_status(201);
                match handler {
                    #(#handle_routes),*
                    _ => {}
                }
            }
        }
    };

    expanded.into()
}
