use proc_macro::TokenStream;
use quote::quote;

use crate::utils::{parse_macro_data, str_to_ident};

struct ControllerArgs {
    routes: Vec<String>,
    middlewares: Vec<syn::Path>,
}

impl syn::parse::Parse for ControllerArgs {
    /// Parses a string like `routes = "get_location, get_location_weather", middlewares = [WeatherGate]`
    /// into a `ControllerArgs` struct.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut routes = vec![];
        let mut middlewares = vec![];

        if input.to_string().starts_with('"') {
            let route = input.parse::<syn::LitStr>()?;
            routes = route
                .value()
                .split(',')
                .map(|r| r.trim().to_string())
                .collect();
            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }
        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;

            match ident.to_string().as_str() {
                "routes" => {
                    if !routes.is_empty() {
                        panic!("routes already registered");
                    }
                    let route = input.parse::<syn::LitStr>()?;
                    routes = route
                        .value()
                        .split(',')
                        .map(|r| r.trim().to_string())
                        .collect();
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
            routes,
            middlewares,
        })
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

    let mut route_registry: Vec<_> = Vec::new();
    let mut handle_routes: Vec<_> = Vec::new();

    args.routes.iter().for_each(|route| {
        let route_ident = str_to_ident(route.to_string());
        let path = str_to_ident(format!("register_{}", route));
        handle_routes.push(quote! {
            #route => {
                Self::#route_ident(self, &req, &mut res).await
            }
        });
        route_registry.push(quote! { controller.#path(); })
    });

    let add_middlewares: Vec<_> = args
        .middlewares
        .iter()
        .map(|m| {
            quote! {
                let middleware: #m = ngyn::NgynProvider.provide();
                middlewares.push(std::sync::Arc::new(middleware));
            }
        })
        .collect();

    let expanded = quote! {
        #(#attrs)*
        #[ngyn::dependency]
        #vis struct #ident {
            routes: Vec<(String, String, String)>,
            middlewares: Vec<std::sync::Arc<dyn ngyn::NgynMiddleware>>,
            #(#fields),*
        }

        #[ngyn::async_trait]
        impl ngyn::NgynController for #ident {
            fn new(middlewares: Vec<std::sync::Arc<dyn ngyn::NgynMiddleware>>) -> Self {
                #(#add_middlewares)*
                let mut controller = #ident {
                    routes: vec![],
                    middlewares,
                    #(#keys: ngyn::NgynProvider.provide()),*
                };
                #(#route_registry)*
                controller
            }

            fn name(&self) -> &str {
                stringify!(#ident)
            }

            fn get_routes(&self) -> Vec<(String, String, String)> {
                self.routes.clone()
            }

            async fn handle(&self, handler: String, req: ngyn::NgynRequest, mut res: ngyn::NgynResponse) -> ngyn::NgynResponse {
                self.middlewares.iter().for_each(|middleware| {
                    middleware.handle(&req, &mut res);
                });
                match handler.as_str() {
                    #(#handle_routes)*
                    _ => {
                        res.set_status(404).clone()
                    }
                }
            }
        }
    };
    expanded.into()
}
