extern crate proc_macro;

mod common {
    pub mod handler;
    pub mod service;
    pub mod state;
}
mod core {
    pub mod dto;
    pub mod param;
    pub mod query;
}

use crate::core::dto::dto_macro;
use crate::core::param::param_macro;
use crate::core::query::query_macro;
use common::handler::handler_macro;
use common::service::service_macro;
use proc_macro::TokenStream;

/// Attribute macro to define a route handler function with optional gates and middlewares
/// options in async functions.
#[proc_macro_attribute]
pub fn handler(args: TokenStream, input: TokenStream) -> TokenStream {
    handler_macro(args, input)
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

#[proc_macro_derive(Service)]
/// The `Service` derive macro is used to generate a Service.
///
/// ### Example
/// ```rust ignore
/// #[derive(Service)]
/// struct MyService {
///     // fields
/// }
/// ```
pub fn service_derive_dto(input: TokenStream) -> TokenStream {
    service_macro(input)
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
