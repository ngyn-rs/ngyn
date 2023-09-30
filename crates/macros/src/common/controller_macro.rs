use proc_macro::TokenStream;
use quote::quote;

use crate::utils::{handle_macro, str_to_ident};

struct ControllerArgs {
    routes: Vec<String>,
    middlewares: Vec<syn::Path>,
}

impl syn::parse::Parse for ControllerArgs {
    /// Parses the attribute arguments of a `#[module]` macro.
    /// We make sure that the arguments are in the format `controllers = [], imports = []`.
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
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let args = syn::parse_macro_input!(args as ControllerArgs);
    let (ident, types, keys) = handle_macro(input);

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
                Self::#route_ident(Self::new(self.middlewares.clone()), req, res).await
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

    let ngyn_controller_alias = str_to_ident(format!("{}ControllerBase", ident));

    let expanded = quote! {
        use ngyn::NgynController as #ngyn_controller_alias;

        #[ngyn::dependency]
        pub struct #ident {
            routes: Vec<(String, String, String)>,
            middlewares: Vec<std::sync::Arc<dyn ngyn::NgynMiddleware>>,
            #(#fields),*
        }

        impl #ident {
            pub fn new(middlewares: Vec<std::sync::Arc<dyn ngyn::NgynMiddleware>>) -> Self {
                #(#add_middlewares)*
                let mut controller = #ident {
                    routes: vec![],
                    middlewares,
                    #(#keys: ngyn::NgynProvider.provide()),*
                };
                #(#route_registry)*
                controller
            }
        }

        #[ngyn::async_trait]
        impl ngyn::NgynController for #ident {
            fn name(&self) -> &str {
                stringify!(#ident)
            }

            fn get_routes(&self) -> Vec<(
                String,
                String,
                String,
            )> {
                self.routes.iter().map(|(path, http_method, handler)| {
                    (path.clone(), http_method.clone(), handler.clone())
                }).collect()
            }

            async fn handle(&self, handler: String, req: ngyn::NgynRequest, res: ngyn::NgynResponse) -> ngyn::NgynResponse {
                for middleware in self.middlewares.clone() {
                    middleware.handle(&req, &res);
                }
                match handler.as_str() {
                    #(#handle_routes)*
                    _ => {
                        res.status(404)
                    }
                }
            }
        }

        impl ngyn::NgynControllerInit for #ident {
            fn new(middlewares: Vec<std::sync::Arc<dyn ngyn::NgynMiddleware>>) -> Self {
                Self::new(middlewares)
            }
        }
    };
    expanded.into()
}
