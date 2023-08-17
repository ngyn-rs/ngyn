use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

/// The `interceptor` attribute is used to implement an `intercept` function for a given type.
/// This function is intended to be used for intercepting and potentially altering the execution of a function.
///
/// # Examples
///
/// ```
/// #[interceptor]
/// struct MyInterceptor;
///
/// let my_interceptor = MyInterceptor;
/// my_interceptor.intercept(&mut some_function);
/// ```
pub fn interceptor_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let name = &input.ident;

    let expanded = quote! {
        impl #name {
            /// The `intercept` function takes a mutable reference to any type.
            /// It is intended to be overridden with logic to intercept and potentially alter the execution of a function.
            pub fn intercept(&self, execution: &mut dyn std::any::Any) {
                // TODO: Implement the logic of the interceptor here
            }
        }
    };

    TokenStream::from(expanded)
}
