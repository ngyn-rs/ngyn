use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn platform_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;

    let expanded = quote! {
        #input

        impl #ident {
            /// Adds a new route to the `NgynApplication` with the `Method::Get`.
            pub fn get<F>(&mut self, path: &str, handler: F) -> &mut Self {
                self.route(path, Method::Get, Box::new(handler))
            }

            /// Adds a new route to the `NgynApplication` with the `Method::Post`.
            pub fn post<F>(&mut self, path: &str, handler: F) -> &mut Self {
                self.route(path, Method::Get, Box::new(handler))
            }

            /// Adds a new route to the `NgynApplication` with the `Method::Put`.
            pub fn put<F>(&mut self, path: &str, handler: F) -> &mut Self {
                self.route(path, Method::Get, Box::new(handler))
            }

            /// Adds a new route to the `NgynApplication` with the `Method::Delete`.
            pub fn delete<F>(&mut self, path: &str, handler: F) -> &mut Self {
                self.route(path, Method::Get, Box::new(handler))
            }

            /// Adds a new route to the `NgynApplication` with the `Method::Patch`.
            pub fn patch<F>(&mut self, path: &str, handler: F) -> &mut Self {
                self.route(path, Method::Get, Box::new(handler))
            }

            /// Adds a new route to the `NgynApplication` with the `Method::Head`.
            pub fn head<F>(&mut self, path: &str, handler: F) -> &mut Self {
                self.route(path, Method::Get, Box::new(handler))
            }
        }
    };

    expanded.into()
}
