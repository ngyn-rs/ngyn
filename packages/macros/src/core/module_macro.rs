extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::utils::handle_macro::handle_macro;

pub fn module_macro(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (ident, types, keys) = handle_macro(input);

    let expanded = quote! {
        use rustle_core::{injectable, RustleModule};

        #[injectable]
        pub struct #ident {
            components: Vec<Box<dyn std::any::Any>>,
            #(#keys: #types),*
        }

        impl RustleModule for #ident {
            /// Creates a new `#ident` with the specified components.
            ///
            /// # Examples
            ///
            /// ```
            /// let module = #ident::new();
            /// ```
            fn new() -> Self {
                #ident {
                    components: vec![],
                    #(#keys: Default::default()),*
                }
            }
        }

        impl #ident {
            /// Returns a reference to a component of type `T` if it exists in the module.
            ///
            /// # Examples
            ///
            /// ```
            /// let my_component: Option<&MyComponent> = module.get();
            /// ```
            pub fn get<T: 'static>(&self) -> Option<&T> {
                for component in &self.components {
                    if let Some(c) = component.downcast_ref::<T>() {
                        return Some(c);
                    }
                }
                None
            }

            /// Adds a component to the module.
            ///
            /// # Examples
            ///
            /// ```
            /// module.add(MyComponent::new());
            /// ```
            pub fn add<T: 'static>(&mut self, component: T) {
                self.components.push(Box::new(component));
            }

            /// Removes a component from the module.
            ///
            /// # Examples
            ///
            /// ```
            /// module.remove::<MyComponent>();
            /// ```
            pub fn remove<T: 'static>(&mut self) {
                self.components.retain(|component| component.downcast_ref::<T>().is_none());
            }
        }
    };
    TokenStream::from(expanded)
}
