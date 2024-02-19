use proc_macro::TokenStream;
use quote::quote;

use crate::utils::parse_macro_data;

struct ControllerArgs {
    middlewares: Vec<syn::Path>,
}

impl syn::parse::Parse for ControllerArgs {
    /// Parses a string like `routes = "get_location, get_location_weather", middlewares = [WeatherGate]`
    /// into a `ControllerArgs` struct.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut middlewares = vec![];

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;

            match ident.to_string().as_str() {
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

        Ok(ControllerArgs { middlewares })
    }
}

pub fn controller_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    let syn::DeriveInput {
        attrs,
        ident,
        data,
        vis,
        ..
    } = syn::parse_macro_input!(input as syn::DeriveInput);
    let args = syn::parse_macro_input!(args as ControllerArgs);
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

    let add_middlewares: Vec<_> = args
        .middlewares
        .iter()
        .map(|m| {
            quote! {
                let middleware: #m = ngyn::prelude::NgynProvider.provide();
                middlewares.push(std::sync::Arc::new(middleware));
            }
        })
        .collect();

    let expanded = quote! {
        #(#attrs)*
        #[ngyn::macros::dependency]
        #vis struct #ident {
            middlewares: Vec<std::sync::Arc<dyn ngyn::prelude::NgynMiddleware>>,
            #(#fields),*
        }

        #[ngyn::prelude::async_trait]
        impl ngyn::prelude::NgynControllerRoutePlaceholder for #ident {
            #[allow(non_upper_case_globals)]
            const routes: &'static [(&'static str, &'static str, &'static str)] = &[];

            async fn __handle_route(
            	&self,
                _handler: &str,
                _req: &mut ngyn::prelude::NgynRequest,
                _res: &mut ngyn::prelude::NgynResponse,
            ) {
                // This is a placeholder for the routing logic of the controller.
            }
        }

        #[ngyn::prelude::async_trait]
        impl ngyn::prelude::NgynController for #ident {
            fn new(middlewares: Vec<std::sync::Arc<dyn ngyn::prelude::NgynMiddleware>>) -> Self {
                #(#add_middlewares)*
                let mut controller = #ident {
                    middlewares,
                    #(#keys: ngyn::prelude::NgynProvider.provide()),*
                };
                controller
            }

            fn routes(&self) -> Vec<(String, String, String)> {
                use ngyn::prelude::NgynControllerRoutePlaceholder;
                Self::routes.iter().map(|(path, method, handler)| {
                    (path.to_string(), method.to_string(), handler.to_string())
                }).collect()
            }

            async fn handle(&self, handler: &str, req: &mut ngyn::prelude::NgynRequest, res: &mut ngyn::prelude::NgynResponse) {
                use ngyn::prelude::NgynControllerRoutePlaceholder;
                self.middlewares.iter().for_each(|middleware| {
                    middleware.handle(req, res);
                });
                self.__handle_route(handler, req, res).await;
            }
        }
    };
    expanded.into()
}
