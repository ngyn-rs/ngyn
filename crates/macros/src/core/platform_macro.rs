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
            pub fn get(&mut self, path: &str, handler: impl ngyn_shared::RouteHandle) -> &mut Self {
                self.route(path, Method::GET, handler.into())
            }

            /// Adds a new route to the `NgynApplication` with the `Method::Post`.
            pub fn post(&mut self, path: &str, handler: impl ngyn_shared::RouteHandle) -> &mut Self {
                self.route(path, Method::POST, handler.into())
            }

            /// Adds a new route to the `NgynApplication` with the `Method::Put`.
            pub fn put(&mut self, path: &str, handler: impl ngyn_shared::RouteHandle) -> &mut Self {
                self.route(path, Method::PUT, handler.into())
            }

            /// Adds a new route to the `NgynApplication` with the `Method::Delete`.
            pub fn delete(&mut self, path: &str, handler: impl ngyn_shared::RouteHandle) -> &mut Self {
                self.route(path, Method::DELETE, handler.into())
            }

            /// Adds a new route to the `NgynApplication` with the `Method::Patch`.
            pub fn patch(&mut self, path: &str, handler: impl ngyn_shared::RouteHandle) -> &mut Self {
                self.route(path, Method::PATCH, handler.into())
            }

            /// Adds a new route to the `NgynApplication` with the `Method::Head`.
            pub fn head(&mut self, path: &str, handler: impl ngyn_shared::RouteHandle) -> &mut Self {
                self.route(path, Method::HEAD, handler.into())
            }
        }
    };

    expanded.into()
}
