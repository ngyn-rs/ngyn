extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::utils::handle_macro::handle_macro;

pub fn module_macro(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (ident, types, keys) = handle_macro(input);

    let default_fields = vec![
        quote! { controllers: Vec<Box<dyn std::any::Any>> },
        quote! { providers: Vec<Box<dyn std::any::Any>> },
    ];

    let fields: Vec<_> = keys
        .iter()
        .zip(types.iter())
        .map(|(key, ty)| quote! { #key: #ty })
        .chain(default_fields.iter().cloned())
        .collect();

    let expanded = quote! {
        use nject::injectable;
        use rustle_core::{RustleInjectable, RustleModule};

        #[injectable]
        pub struct #ident {
            #(#fields),*
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
                    controllers: vec![],
                    providers: vec![],
                    #(#keys: RustleInjectable::new()),*
                }
            }
        }

        impl #ident {
            /// Returns a reference to a controller of type `T` if it exists in the module.
            ///
            /// # Examples
            ///
            /// ```
            /// let my_controller: Option<&MyController> = module.get_controller();
            /// ```
            pub fn get_controller<T: 'static>(&self) -> Option<&T> {
                for controller in &self.controllers {
                    if let Some(c) = controller.downcast_ref::<T>() {
                        return Some(c);
                    }
                }
                None
            }

            /// Returns a reference to a provider of type `T` if it exists in the module.
            ///
            /// # Examples
            ///
            /// ```
            /// let my_provider: Option<&MyProvider> = module.get_provider();
            /// ```
            pub fn get_provider<T: 'static>(&self) -> Option<&T> {
                for provider in &self.providers {
                    if let Some(p) = provider.downcast_ref::<T>() {
                        return Some(p);
                    }
                }
                None
            }

            /// Adds a controller to the module.
            ///
            /// # Examples
            ///
            /// ```
            /// module.add_controller(MyController::new());
            /// ```
            pub fn add_controller<T: 'static>(&mut self, controller: T) {
                self.controllers.push(Box::new(controller));
            }

            /// Adds a provider to the module.
            ///
            /// # Examples
            ///
            /// ```
            /// module.add_provider(MyProvider::new());
            /// ```
            pub fn add_provider<T: 'static>(&mut self, provider: T) {
                self.providers.push(Box::new(provider));
            }

            /// Removes a controller from the module.
            ///
            /// # Examples
            ///
            /// ```
            /// module.remove_controller::<MyController>();
            /// ```
            pub fn remove_controller<T: 'static>(&mut self) {
                self.controllers.retain(|controller| controller.downcast_ref::<T>().is_none());
            }

            /// Removes a provider from the module.
            ///
            /// # Examples
            ///
            /// ```
            /// module.remove_provider::<MyProvider>();
            /// ```
            pub fn remove_provider<T: 'static>(&mut self) {
                self.providers.retain(|provider| provider.downcast_ref::<T>().is_none());
            }
        }
    };
    TokenStream::from(expanded)
}
