extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn module_macro(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        use rustle_core::{provider, RustleModule};

        #[provider]
        pub struct #name {
            components: Vec<Box<dyn std::any::Any>>,
        }

        impl RustleModule for #name {
            /// Creates a new `#name` with the specified components.
            ///
            /// # Examples
            ///
            /// ```
            /// let module = #name::new();
            /// ```
            fn new() -> Self {
                #name {
                    components: vec![],
                }
            }
        }

        impl #name {
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
