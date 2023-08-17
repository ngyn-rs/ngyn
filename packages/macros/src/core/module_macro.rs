extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Type};

pub fn module_macro(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    let raw_fields = match &data {
        syn::Data::Struct(d) => &d.fields,
        _ => panic!("Only structs are supported"),
    };
    let types = raw_fields.iter().map(|f| &f.ty).collect::<Vec<&Type>>();
    let keys = raw_fields
        .iter()
        .map(|f| f.ident.as_ref())
        .filter_map(|i| i)
        .collect::<Vec<&Ident>>();

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
