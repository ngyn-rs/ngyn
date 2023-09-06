extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::utils::handle_macro::handle_macro;

struct ModuleArgs {
    controllers: Vec<syn::Path>,
}

pub fn module_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (ident, _, _) = handle_macro(input);

    // Parse the attributes to get the controller types
    let args = {
        let input_str = args.to_string();

        let controllers = if input_str.starts_with('[') && input_str.ends_with(']') {
            let controllers: Vec<syn::Path> =
                if !input_str[1..input_str.len() - 1].trim().is_empty() {
                    input_str[1..input_str.len() - 1]
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .map(|s| syn::parse_str(&s).unwrap())
                        .collect()
                } else {
                    vec![]
                };

            Some(controllers)
        } else if input_str.is_empty() {
            Some(vec![])
        } else {
            panic!("invalid path")
        };

        ModuleArgs {
            controllers: controllers.unwrap(),
        }
    };

    let add_controllers: Vec<_> = args
        .controllers
        .iter()
        .map(|controller| {
            quote! {
                let controller: #controller = #controller::new();
                controllers.push(std::sync::Arc::new(controller));
            }
        })
        .collect();

    let keys = args.controllers.iter().map(|controller| {
        let segments = &controller.segments;
        let name = &segments.last().unwrap().ident;
        quote! { #name }
    });

    let fields: Vec<_> = keys
        .clone()
        .zip(args.controllers.iter())
        .map(|(key, controller)| {
            quote! {
                #key: #controller
            }
        })
        .collect();

    let expanded = quote! {
        #[ngyn::dependency]
        pub struct #ident {
            #(#fields),*
        }

        impl ngyn::NgynModule for #ident {
            /// Creates a new `#ident` with the specified components.
            ///
            /// # Examples
            ///
            /// ```
            /// let module = #ident::new();
            /// ```
            fn new() -> Self {
                #ident {
                    #(#keys: ngyn::NgynProvider.provide()),*
                }
            }

            /// Returns the controllers of the module.
            ///
            /// # Examples
            ///
            /// ```
            /// let module = #ident::new();
            /// let controllers = module.get_controllers();
            /// ```
            fn get_controllers(&self) -> Vec<std::sync::Arc<dyn ngyn::NgynController>> {
                let mut controllers: Vec<std::sync::Arc<dyn ngyn::NgynController>> = vec![];
                #(#add_controllers)*
                controllers
            }
        }
    };
    TokenStream::from(expanded)
}
