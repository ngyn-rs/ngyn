use proc_macro::TokenStream;
use quote::quote;

use crate::utils::parse_macro_data;

struct ControllerArgs {
    prefix: Option<syn::LitStr>,
    middlewares: Vec<syn::Path>,
    init: Option<syn::LitStr>,
}

impl syn::parse::Parse for ControllerArgs {
    /// Parses a string like `prefix="/weather", middlewares = [WeatherGate]`
    /// into a `ControllerArgs` struct.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut middlewares = vec![];
        let mut prefix = None;
        let mut init = None;

        // match the input with format "/prefix"
        if !input.is_empty() && input.peek(syn::LitStr) {
            let path: syn::LitStr = input.parse()?;
            prefix = Some(path);

            return Ok(ControllerArgs {
                middlewares,
                prefix,
                init,
            });
        }

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;

            match ident.to_string().as_str() {
                "prefix" => {
                    let path: syn::LitStr = input.parse()?;
                    prefix = Some(path);
                }
                "middlewares" => {
                    let content;
                    syn::bracketed!(content in input);
                    while !content.is_empty() {
                        let path: syn::Path = content.parse()?;
                        middlewares.push(path);
                        if !content.is_empty() {
                            content.parse::<syn::Token![,]>()?;
                        }
                    }
                }
                "init" => {
                    init = input.parse()?;
                }
                _ => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("unexpected attribute `{}`", ident),
                    ));
                }
            }
            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(ControllerArgs {
            middlewares,
            prefix,
            init,
        })
    }
}

pub fn controller_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    let syn::DeriveInput {
        attrs,
        ident,
        data,
        vis,
        generics,
    } = syn::parse_macro_input!(input as syn::DeriveInput);
    let ControllerArgs {
        middlewares,
        prefix,
        init,
    } = syn::parse_macro_input!(args as ControllerArgs);
    let controller_fields = parse_macro_data(data);

    let fields: Vec<_> = controller_fields
        .iter()
        .map(
            |syn::Field {
                 ident,
                 ty,
                 vis,
                 attrs,
                 colon_token,
                 ..
             }| {
                quote! {
                    #(#attrs),* #vis #ident #colon_token #ty
                }
            },
        )
        .collect();

    let add_fields: Vec<_> = controller_fields
        .iter()
        .map(
            |syn::Field {
                 ident,
                 ty,
                 colon_token,
                 ..
             }| {
                quote! {
                    #ident #colon_token #ty::default()
                }
            },
        )
        .collect();

    let add_middlewares: Vec<_> = middlewares
        .iter()
        .map(|m| {
            quote! {
                let middleware = #m::default();
                middlewares.push(std::sync::Arc::new(middleware));
            }
        })
        .collect();

    let path_prefix = {
        if let Some(prefix) = prefix {
            quote! {
                #prefix.to_string() + "/"
            }
        } else {
            quote! {
                "".to_string()
            }
        }
    };

    let init_controller = {
        if let Some(init) = init {
            let init_ident = init.parse::<syn::Ident>().unwrap();
            quote! {
                #ident::#init_ident()
            }
        } else {
            quote! {
                #ident {
                    #(#add_fields),*
                }
            }
        }
    };

    let expanded = quote! {
        #(#attrs)*
        #vis struct #ident #generics {
            #(#fields),*
        }

        #[ngyn::prelude::async_trait]
        impl #generics ngyn::prelude::NgynControllerHandler for #ident #generics {
            #[allow(non_upper_case_globals)]
            const routes: &'static [(&'static str, &'static str, &'static str)] = &[];

            async fn __handle_route(
                &self,
                _handler: &str,
                _cx: &mut ngyn::prelude::NgynContext,
                _res: &mut ngyn::prelude::NgynResponse,
            ) {
                // This is a placeholder for the routing logic of the controller.
            }
        }

        #[ngyn::prelude::async_trait]
        impl #generics ngyn::prelude::NgynController for #ident #generics {
            fn new() -> Self {
                #init_controller
            }

            fn routes(&self) -> Vec<(String, String, String)> {
                use ngyn::prelude::NgynControllerHandler;
                Self::routes.iter().map(|(path, method, handler)| {
                    ((#path_prefix + path).replace("//", "/"), method.to_string(), handler.to_string())
                }).collect()
            }

            async fn handle(
                &self, handler: &str,
                cx: &mut ngyn::prelude::NgynContext,
                res: &mut ngyn::prelude::NgynResponse,
            ) {
                use ngyn::prelude::NgynControllerHandler;
                let mut middlewares: Vec<std::sync::Arc<dyn ngyn::prelude::NgynMiddleware>> = vec![];
                #(#add_middlewares)*
                middlewares.iter().for_each(|middleware| {
                    middleware.handle(cx, res);
                });
                self.__handle_route(handler, cx, res).await;
            }
        }
    };
    expanded.into()
}
