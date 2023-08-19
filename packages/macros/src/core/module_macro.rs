extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::utils::handle_macro::handle_macro;

pub fn module_macro(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (ident, types, keys) = handle_macro(input);

    let default_fields = vec![
        quote! { controllers: Vec<std::sync::Arc<dyn rustle_core::RustleController>> },
        quote! { providers: Vec<std::sync::Arc<dyn rustle_core::RustleInjectable>> },
    ];

    let fields: Vec<_> = keys
        .iter()
        .zip(types.iter())
        .map(|(key, ty)| quote! { #key: #ty })
        .chain(
            default_fields
                .iter()
                .filter(|default| {
                    !keys
                        .iter()
                        .any(|key| default.to_string().contains(&key.to_string()))
                })
                .cloned(),
        )
        .collect();

    let expanded = quote! {
        #[nject::injectable]
        pub struct #ident {
            #(#fields),*
        }

        impl rustle_core::RustleModule for #ident {
            /// Creates a new `#ident` with the specified components.
            ///
            /// # Examples
            ///
            /// ```
            /// let module = #ident::new();
            /// ```
            fn new() -> Self {
                #ident {
                    controllers: vec![],
                    providers: vec![],
                    // #(#keys: RustleInjectable::new()),*
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
            fn get_controllers(&self) -> Vec<std::sync::Arc<dyn rustle_core::RustleController>> {
                self.controllers
                    .clone()
                    .into_iter()
                    .map(|provider| std::sync::Arc::clone(&provider) as std::sync::Arc<dyn rustle_core::RustleController>)
                    .collect::<Vec<std::sync::Arc<dyn rustle_core::RustleController>>>()
            }

            /// Returns the providers of the module.
            ///
            /// # Examples
            ///
            /// ```
            /// let module = #ident::new();
            /// let providers = module.get_providers();
            /// ```
            fn get_providers(&self) -> Vec<std::sync::Arc<dyn rustle_core::RustleInjectable>> {
                self.providers
                    .clone()
                    .into_iter()
                    .map(|provider| std::sync::Arc::clone(&provider) as std::sync::Arc<dyn rustle_core::RustleInjectable>)
                    .collect::<Vec<std::sync::Arc<dyn rustle_core::RustleInjectable>>>()
            }
        }
    };
    TokenStream::from(expanded)
}
