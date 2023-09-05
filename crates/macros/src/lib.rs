extern crate proc_macro;

mod common;
mod core;
mod utils;

use crate::common::{controller_macro::*, injectable_macro::*, route_get_macro::*};
use crate::core::{interceptor_macro::*, module_macro::*};
use proc_macro::TokenStream;

#[proc_macro_attribute]
/// `Module` is a procedural macro that generates a struct and its implementation.
/// The struct `Module` contains a vector of boxed dynamic components.
///
/// ### Examples
///
/// ```
/// #[module]
/// struct MyModule;
/// let module = MyModule::new();
/// let my_component: Option<&MyComponent> = module.get();
/// ```
pub fn module(args: TokenStream, input: TokenStream) -> TokenStream {
    module_macro(args, input)
}

#[proc_macro_attribute]
/// The `interceptor` attribute is used to implement an `intercept` function for a given type.
/// This function is intended to be used for intercepting and potentially altering the execution of a function.
///
/// ### Examples
///
/// ```
/// #[interceptor]
/// struct MyInterceptor;
///
/// let my_interceptor = MyInterceptor;
/// my_interceptor.intercept(&mut some_function);
/// ```
pub fn interceptor(args: TokenStream, input: TokenStream) -> TokenStream {
    interceptor_macro(args, input)
}

#[proc_macro_attribute]
/// The `injectable` attribute is used to mark a struct as injectable.
/// This means that the struct can be automatically provided as a dependency where needed.
///
/// ### Examples
///
/// ```
/// #[injectable]
/// struct MyService;
///
/// let my_service = MyService::new();
/// ```
pub fn injectable(args: TokenStream, input: TokenStream) -> TokenStream {
    injectable_macro(args, input)
}

#[proc_macro_attribute]
/// The `controller` attribute is used to mark a struct as a controller.
/// This means that the struct can handle HTTP requests and responses.
///
/// ### Examples
///
/// ```
/// #[controller]
/// struct MyController;
///
/// let my_controller = MyController::new();
/// ```
pub fn controller(args: TokenStream, input: TokenStream) -> TokenStream {
    controller_macro(args, input)
}

#[proc_macro_attribute]
/// The `get` attribute is used to mark a function as a GET route.
///
/// ### Examples
///
/// ```
/// #[get("/")]
/// fn index() -> String {
///    "Hello, world!".to_string()
/// }
/// ```
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    route_get_macro(args, input)
}