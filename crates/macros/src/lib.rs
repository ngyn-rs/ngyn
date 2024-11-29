extern crate proc_macro;

mod common;
mod core;
mod utils;

use crate::common::check::check_fn_macro;
use crate::common::injectable::*;
use crate::core::dto::dto_macro;
use crate::core::param::param_macro;
use crate::core::query::query_macro;
use common::check::check_impl_macro;
use common::handler::handler_macro;
use common::inject::inject_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn handler(args: TokenStream, input: TokenStream) -> TokenStream {
    handler_macro(args, input)
}

#[proc_macro_attribute]
/// `injectable` attribute is used to mark a struct as injectable.
///
/// ### Arguments
/// * `init` - The name of the init method that will be called when the struct is initialized.
///
/// ### Example
/// ```rust ignore
/// #[injectable]
/// struct MyStruct {
///     // struct fields and methods
/// }
/// ```
pub fn injectable(args: TokenStream, input: TokenStream) -> TokenStream {
    injectable_macro(args, input)
}

#[proc_macro_attribute]
/// The `inject` attribute is used to mark a field as injectable.
/// The field must be a struct that is marked with the [`injectable`] attribute.
pub fn inject(args: TokenStream, input: TokenStream) -> TokenStream {
    inject_macro(args, input)
}

#[proc_macro_attribute]
/// The `check` macro is used to determine if a route should be executed.
/// If the gate returns false, the route will not be executed.
///
/// ### Arguments
///
/// * `CheckGate` - The path to the gate that will be used to determine if the route should be executed.
///
/// ### Panics
///
/// Panics if the attribute is not used on a method or an impl block.
///
/// ### Example
/// ```rust ignore
/// #[check(CheckGate)]
/// fn my_route(&self) {
///     // route implementation
/// }
/// ```
pub fn check(args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_input = syn::parse::<syn::Item>(input);
    match parsed_input {
        Ok(syn::Item::Fn(input)) => check_fn_macro(args, input),
        Ok(syn::Item::Impl(impl_item)) => check_impl_macro(args, impl_item),
        _ => panic!("`check` attribute can only be used on methods or impl blocks"),
    }
}

#[proc_macro_derive(Dto)]
/// The `Dto` derive macro is used to generate a DTO struct.
///
/// ### Example
/// ```rust ignore
/// #[derive(Dto)]
/// struct MyDto {
///     // fields
/// }
/// ```
pub fn dto_derive_dto(input: TokenStream) -> TokenStream {
    dto_macro(input)
}

#[proc_macro_derive(Query)]
/// The `Query` derive macro is used to derive a struct that can be used to parse query parameters.
///
/// ### Example
/// ```rust ignore
/// #[derive(Query)]
/// struct MyQuery {
///    page: u32,
///   limit: u32,
/// }
/// ```
pub fn query_derive_macro(input: TokenStream) -> TokenStream {
    query_macro(input)
}

#[proc_macro_derive(Param)]
/// The `Param` derive macro is used to derive a struct that can be used to parse route parameters.
///
/// ### Example
/// ```rust ignore
/// #[derive(Param)]
/// struct MyParam {
///   id: u32,
/// }
/// ```
pub fn param_derive_macro(input: TokenStream) -> TokenStream {
    param_macro(input)
}

#[proc_macro_derive(AppState)]
/// The `AppState` derive macro is used to derive a struct that can be used as a state in a server.
///
/// ### Example
/// ```rust ignore
/// #[derive(AppState)]
/// struct MyState {
///    // fields
/// }
/// ```
pub fn app_state_derive_macro(input: TokenStream) -> TokenStream {
    common::state::derive_app_state_macro(input)
}
