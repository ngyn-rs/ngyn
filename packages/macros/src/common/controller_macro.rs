use proc_macro::TokenStream;
use quote::quote;

use crate::utils::{handle_macro, str_to_ident};

pub fn controller_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let (ident, types, keys) = handle_macro(input);

    let fields: Vec<_> = types
        .iter()
        .zip(keys.iter())
        .map(|(ty, key)| {
            quote! {
                #[allow(dead_code)]
                #key: #ty
            }
        })
        .collect();

    let arg: Option<String> = {
        let input_str = args.to_string();
        // TODO: catch invalid arguments
        if input_str.starts_with("\"") && input_str.ends_with("\"") {
            Some(input_str.trim_matches('"').to_lowercase())
        } else {
            None
        }
    };
    let mut route_registry: Vec<_> = Vec::new();
    let mut handle_routes: Vec<_> = Vec::new();

    match arg {
        Some(routes) => routes.split(",").into_iter().for_each(|route| {
            let route_ident = str_to_ident(route.trim().to_string());
            let path = str_to_ident(format!("register_{}", route.trim()));
            handle_routes.push(quote! {
                    #route => {
                        Self::#route_ident(Self::new(), req, res)
                    }
            });
            route_registry.push(quote! { controller.#path(); })
        }),
        _ => {}
    }

    let rustle_controller_alias = str_to_ident(format!("{}ControllerBase", ident));

    let expanded = quote! {
        use rustle_core::RustleController as #rustle_controller_alias;

        #[rustle_core::dependency]
        pub struct #ident {
            all_routes: Vec<(
                    String,
                    String,
                    Box<
                        dyn Fn(rustle_core::RustleRequest, rustle_core::RustleResponse) -> rustle_core::RustleResponse
                            + Send
                            + Sync,
                    >,
                )>,
            #(#fields),*
        }

        impl #ident {
            pub fn new() -> Self {
                let mut controller = #ident {
                    all_routes: vec![],
                    #(#keys: rustle_core::RustleProvider.provide()),*
                };
                #(#route_registry)*
                controller
            }
        }

        impl rustle_core::RustleController for #ident {
            fn name(&self) -> &str {
                stringify!(#ident)
            }

            fn add_route(
                &mut self,
                path: String,
                http_method: String,
                handler: String,
            ) {
                self.all_routes.push((path, http_method, Box::new(|req, res| res)));
            }

            fn routes(&self) -> Vec<(
                String,
                String,
                String,
            )> {
                self.all_routes.iter().map(|(path, http_method, handler)| {
                    (path.clone(), http_method.clone(), path.clone())
                }).collect()
            }

            fn handle(&self, handler: String, req: rustle_core::RustleRequest, res: rustle_core::RustleResponse) -> rustle_core::RustleResponse {
                match handler.as_str() {
                    #(#handle_routes)*
                    _ => {
                        res
                    }
                }
            }
        }

        impl rustle_core::RustleControllerInit for #ident {
            fn new() -> Box<dyn rustle_core::RustleController> {
                Box::new(Self::new())
            }
        }
    };
    expanded.into()
}
