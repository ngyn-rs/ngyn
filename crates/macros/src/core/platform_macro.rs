use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn platform_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;

    let expanded = quote! {
        #input

        impl #ident {
            /// Adds a new route to the `NgynApplication` with the `HttpMethod::Get`.
            pub fn get<F>(&mut self, path: &str, handler: F) -> &mut Self {
                self.route(path, HttpMethod::Get, Box::new(handler))
            }

            /// Adds a new route to the `NgynApplication` with the `HttpMethod::Post`.
            pub fn post<F>(&mut self, path: &str, handler: F) -> &mut Self {
                self.route(path, HttpMethod::Get, Box::new(handler))
            }

            /// Adds a new route to the `NgynApplication` with the `HttpMethod::Put`.
            pub fn put<F>(&mut self, path: &str, handler: F) -> &mut Self {
                self.route(path, HttpMethod::Get, Box::new(handler))
            }

            /// Adds a new route to the `NgynApplication` with the `HttpMethod::Delete`.
            pub fn delete<F>(&mut self, path: &str, handler: F) -> &mut Self {
                self.route(path, HttpMethod::Get, Box::new(handler))
            }

            /// Adds a new route to the `NgynApplication` with the `HttpMethod::Patch`.
            pub fn patch<F>(&mut self, path: &str, handler: F) -> &mut Self {
                self.route(path, HttpMethod::Get, Box::new(handler))
            }

            /// Adds a new route to the `NgynApplication` with the `HttpMethod::Head`.
            pub fn head<F>(&mut self, path: &str, handler: F) -> &mut Self {
                self.route(path, HttpMethod::Get, Box::new(handler))
            }
        }
    };

    expanded.into()
}
