use ngyn_shared::{HttpMethod, Path};
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemImpl;

pub struct PathArg {
    pub path: Option<Path>,
}

impl syn::parse::Parse for PathArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = if input.peek(syn::LitStr) {
            let path = input.parse::<syn::LitStr>()?;
            Some(Path::Single(path.value()))
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
            Some(Path::Multiple(paths))
        } else {
            None
        };

        Ok(PathArg { path })
    }
}

pub struct Args {
    pub path: Option<Path>,
    pub http_method: String,
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let http_method = input.parse::<syn::LitStr>()?.value();

        if HttpMethod::from(&http_method) == HttpMethod::Unknown {
            panic!("Unsupported HTTP method: {}", http_method)
        } else {
            input.parse::<syn::Token![,]>()?;
        }

        let PathArg { path } = input.parse::<PathArg>()?;

        Ok(Args { path, http_method })
    }
}

pub fn routes_macro(raw_input: TokenStream) -> TokenStream {
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

    match trait_ {
        Some((..)) => panic!("Trait impls are not supported"),
        None => quote! {},
    };

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
                            let Args { path, http_method } = attr.parse_args::<Args>().unwrap();
                            (path, http_method)
                        } else {
                            let PathArg { path } = attr.parse_args::<PathArg>().unwrap();
                            let http_method = attr.path().get_ident().unwrap().to_string();
                            (path, http_method)
                        }
                    };
                    path.unwrap().each(|path| {
                        let ident = method.sig.ident.clone();
                        let ident_str = ident.to_string();
                        route_defs.push(quote! {(
                            #path,
                            #http_method,
                            #ident_str,
                        )});
                        handle_routes.push(quote! {
                            #ident_str => {
                                let body = self.#ident(cx, res).await;
                                res.peek(body.parse_body());
                            }
                        });
                    })
                }
            }
        }
    }

    let expanded = quote! {
        #defaultness #unsafety #(#attrs)*
        #impl_token #generics #self_ty {
            #[allow(non_upper_case_globals)]
            const routes: &'static [(&'static str, &'static str, &'static str)] = &[
                #(#route_defs),*
            ];
            #(#items)*

            async fn __handle_route(
                &self,
                handler: &str,
                cx: &mut ngyn::prelude::NgynContext,
                res: &mut ngyn::prelude::NgynResponse
            ) {
                match handler {
                    #(#handle_routes),*
                    _ => {
                        res.set_status(404);
                    }
                }
            }
        }
    };

    expanded.into()
}
