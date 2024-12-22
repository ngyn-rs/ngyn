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

/// Attribute macro to define a route handler function with optional gates and middlewares.
///
/// # Usage
///
/// The `#[handler]` macro is used to mark a function as a handler. You can optionally specify
/// `gates` and `middlewares` as attributes to configure behavior.
///
/// ```
/// #[handler(gates = [Gate1, Gate2], middlewares = [Middleware1, Middleware2])]
/// async fn my_handler() {
///     // Your handler logic here
/// }
/// ```
///
/// # Parameters
///
/// - `gates`: A list of gate types to be checked before executing the handler. Each gate must implement
///   the `NgynGate` trait and provide the `can_activate` method.
///
/// - `middlewares`: A list of middleware types to be applied to the context. Each middleware must
///   implement the `NgynMiddleware` trait and provide the `handle` method.
///
/// # Notes
///
/// - Gates are executed in the order provided in the `gates` list. If any gate fails or `can_activate` returns false,
///   the handler will not execute.
/// - Middlewares are executed sequentially in the order provided in the `middlewares` list.
/// - `#[handler]` generates the necessary boilerplate code to invoke gates and middlewares as part of
///   the handler's execution.
/// - It also provides arguments to the handler function that are automatically transformed from the context.
///
/// # Example
///
/// ```
/// #[handler(gates = [AuthGate], middlewares = [LoggerMiddleware])]
/// async fn protected_handler() {
///     // Handler logic for authorized requests
/// }
/// ```
///
/// # Errors
///
/// - Compilation will fail if a specified gate or middleware does not implement the required traits.
/// - Ensure that all gate and middleware types used are properly imported and available in the scope.
///
/// # See Also
///
/// - [`NgynGate`](https://docs.rs/ngyn/latest/ngyn/prelude/trait.NgynGate.html)
/// - [`NgynMiddleware`](https://docs.rs/ngyn/latest/ngyn/prelude/trait.NgynMiddleware.html)
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
