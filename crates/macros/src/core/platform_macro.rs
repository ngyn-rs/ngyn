use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn platform_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;

    let expanded = quote! {
        #input

        impl #ident {
            /// Adds a new route to the App with the `HttpMethod::Get`.
            pub fn get(&mut self, path: &str, handler: impl Handler) -> &mut Self {
                self.route(path, HttpMethod::Get, Box::new(handler))
            }

            /// Adds a new route to the App with the `HttpMethod::Post`.
            pub fn post(&mut self, path: &str, handler: impl Handler) -> &mut Self {
                self.route(path, HttpMethod::Get, Box::new(handler))
            }

            /// Adds a new route to the App with the `HttpMethod::Put`.
            pub fn put(&mut self, path: &str, handler: impl Handler) -> &mut Self {
                self.route(path, HttpMethod::Get, Box::new(handler))
            }

            /// Adds a new route to the App with the `HttpMethod::Delete`.
            pub fn delete(&mut self, path: &str, handler: impl Handler) -> &mut Self {
                self.route(path, HttpMethod::Get, Box::new(handler))
            }

            /// Adds a new route to the App with the `HttpMethod::Patch`.
            pub fn patch(&mut self, path: &str, handler: impl Handler) -> &mut Self {
                self.route(path, HttpMethod::Get, Box::new(handler))
            }

            /// Adds a new route to the App with the `HttpMethod::Head`.
            pub fn head(&mut self, path: &str, handler: impl Handler) -> &mut Self {
                self.route(path, HttpMethod::Get, Box::new(handler))
            }
        }
    };

    expanded.into()
}
