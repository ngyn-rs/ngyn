use proc_macro::TokenStream;
use quote::quote;

use crate::utils::parse_macro_data;

struct ControllerArgs {
    prefix: Option<syn::LitStr>,
    middlewares: Vec<syn::Path>,
}

impl syn::parse::Parse for ControllerArgs {
    /// Parses a string like `prefix="/weather", middlewares = [WeatherGate]`
    /// into a `ControllerArgs` struct.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut middlewares = vec![];
        let mut prefix = None;

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
    } = syn::parse_macro_input!(args as ControllerArgs);
    let (types, keys) = parse_macro_data(data);

    let fields: Vec<_> = types
        .iter()
        .zip(keys.iter())
        .map(|(ty, key)| {
            quote! {
                #key: #ty
            }
        })
        .collect();

    let add_middlewares: Vec<_> = middlewares
        .iter()
        .map(|m| {
            quote! {
                let middleware: #m = ngyn::prelude::NgynProvider.provide();
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

    let expanded = quote! {
        #(#attrs)*
        #[ngyn::macros::dependency]
        #vis struct #ident #generics {
            data: ngyn::prelude::NgynControllerData,
            #(#fields),*
        }

        #[ngyn::prelude::async_trait]
        impl #generics ngyn::prelude::NgynControllerRoutePlaceholder for #ident #generics {
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
            fn new(middlewares: Vec<std::sync::Arc<dyn ngyn::prelude::NgynMiddleware>>) -> Self {
                #(#add_middlewares)*
                let mut controller = #ident {
                    data: ngyn::prelude::NgynControllerData::new(middlewares),
                    #(#keys: ngyn::prelude::NgynProvider.provide()),*
                };
                controller
            }

            fn routes(&self) -> Vec<(String, String, String)> {
                use ngyn::prelude::NgynControllerRoutePlaceholder;
                Self::routes.iter().map(|(path, method, handler)| {
                    (#path_prefix + path, method.to_string(), handler.to_string())
                }).collect()
            }

            async fn handle(&self, handler: &str, cx: &mut ngyn::prelude::NgynContext, res: &mut ngyn::prelude::NgynResponse) {
                use ngyn::prelude::NgynControllerRoutePlaceholder;
                self.data.middlewares().iter().for_each(|middleware| {
                    middleware.handle(cx, res);
                });
                self.__handle_route(handler, cx, res).await;
            }
        }
    };
    expanded.into()
}
