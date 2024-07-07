extern crate proc_macro;

mod common;
mod core;
mod utils;

use crate::common::check_macro::check_fn_macro;
use crate::common::{controller_macro::*, injectable_macro::*, route_macro::*, routes_macro::*};
use crate::core::dto::dto_macro;
use crate::core::module::*;
use common::check_macro::check_impl_macro;
use common::inject_macro::inject_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
/// `module` marks a struct that contains controllers and imports other modules.
///
/// ##### Arguments
/// * `controllers` - A list of controllers that the module will contain.
/// * `imports` - A list of modules that the module will import.
/// * `init` - The name of the init method that will be called when the module is initialized.
///
/// ##### Example
/// ```rust ignore
/// #[module(controllers = [MyController1, MyController2])]
/// struct MyModule;
/// ```
///
/// ##### Example with imports
/// ```rust ignore
/// #[module(controllers = [MyController1, MyController2], imports = [MyModule1, MyModule2])]
/// struct MyModule;
/// ```
///
/// ##### Example with init method
/// ```rust ignore
/// #[module(controllers = [MyController], imports = [MyModule], init = "start")]
/// struct MyModule;
///
/// impl MyModule {
///    fn start() {
///       // init method implementation
///   }
/// }
/// ```
pub fn module(args: TokenStream, input: TokenStream) -> TokenStream {
    module_macro(args, input)
}

#[proc_macro_attribute]
/// The `injectable` attribute is used to mark a struct as injectable.
///
/// ##### Arguments
/// * `init` - The name of the init method that will be called when the struct is initialized.
///
/// ##### Example
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
/// The field must be a struct that is marked with the `injectable` attribute.
pub fn inject(args: TokenStream, input: TokenStream) -> TokenStream {
    inject_macro(args, input)
}

#[proc_macro_attribute]
/// The `controller` attribute is used to mark a struct as a controller.
///
/// ##### Example
/// ```rust ignore
/// #[controller]
/// struct MyController {
///     // controller fields and methods
/// }
/// ```
pub fn controller(args: TokenStream, input: TokenStream) -> TokenStream {
    controller_macro(args, input)
}

#[proc_macro_attribute]
/// The `routes` attribute is used to mark a struct impl as a routes container.
///
/// ##### Example
/// ```rust ignore
/// #[controller]
/// struct MyController {
///     // controller fields and methods
/// }
///
/// #[routes]
/// impl MyController {
///     // route methods
/// }
/// ```
pub fn routes(_args: TokenStream, input: TokenStream) -> TokenStream {
    routes_macro(input)
}

#[proc_macro_attribute]
/// The `route` attribute is used to mark a method of a controller impl as a route.
///
/// ##### Arguments
/// * `method` - The HTTP method that the route will handle. (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS)
/// * `path` - The path that the route will handle.
///
/// ##### Example
/// ```rust ignore
/// #[controller]
/// struct MyController {
///     // controller fields and methods
/// }
///
/// #[routes]
/// impl MyController {
///     #[route("GET", "/users")]
///     fn get_users(&self) {
///         // route implementation
///     }
/// }
/// ```
pub fn route(args: TokenStream, input: TokenStream) -> TokenStream {
    route_macro(args, input)
}

#[proc_macro_attribute]
/// The `get` attribute is used to mark a controller method as a GET route.
///
/// ##### Arguments
/// * `path` - The path that the route will handle.
///
/// ##### Example
/// ```rust ignore
/// #[controller]
/// struct MyController {
///     // controller fields and methods
/// }
///
/// #[routes]
/// impl MyController {
///     #[get("/users")]
///     fn get_users(&self) {
///         // route implementation
///     }
/// }
/// ```
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_str::<syn::Expr>(args.to_string().as_str()).unwrap();
    let args_with_method = quote::quote! { "GET", #args };
    route_macro(args_with_method.into(), input)
}

#[proc_macro_attribute]
/// The `post` attribute is used to mark a controller method as a POST route.
///
/// ##### Arguments
/// * `path` - The path that the route will handle.
///
/// ##### Example
/// ```rust ignore
/// #[controller]
/// struct MyController {
///     // controller fields and methods
/// }
///
/// #[routes]
/// impl MyController {
///     #[post("/users")]
///     fn create_user(&self) {
///         // route implementation
///     }
/// }
/// ```
pub fn post(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_str::<syn::Expr>(args.to_string().as_str()).unwrap();
    let args_with_method = quote::quote! { "POST", #args };
    route_macro(args_with_method.into(), input)
}

#[proc_macro_attribute]
/// The `put` attribute is used to mark a controller method as a PUT route.
///
/// ##### Arguments
/// * `path` - The path that the route will handle.
///
/// ##### Example
/// ```rust ignore
/// #[controller]
/// struct MyController {
///     // controller fields and methods
/// }
///
/// #[routes]
/// impl MyController {
///     #[put("/users")]
///     fn update_user(&self) {
///         // route implementation
///     }
/// }
/// ```
pub fn put(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_str::<syn::Expr>(args.to_string().as_str()).unwrap();
    let args_with_method = quote::quote! { "PUT", #args };
    route_macro(args_with_method.into(), input)
}

#[proc_macro_attribute]
/// The `delete` attribute is used to mark a controller method as a DELETE route.
///
/// ##### Arguments
/// * `path` - The path that the route will handle.
///
/// ##### Example
/// ```rust ignore
/// #[controller]
/// struct MyController {
///     // controller fields and methods
/// }
///
/// #[routes]
/// impl MyController {
///     #[delete("/users")]
///     fn delete_user(&self) {
///         // route implementation
///     }
/// }
/// ```
pub fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_str::<syn::Expr>(args.to_string().as_str()).unwrap();
    let args_with_method = quote::quote! { "DELETE", #args };
    route_macro(args_with_method.into(), input)
}

#[proc_macro_attribute]
/// The `patch` attribute is used to mark a method of a controller impl as a PATCH route.
///
/// ##### Arguments
/// * `path` - The path that the route will handle.
///
/// ##### Example
/// ```rust ignore
/// #[controller]
/// struct MyController {
///     // controller fields and methods
/// }
///
/// #[routes]
/// impl MyController {
///     #[patch("/users")]
///     fn update_user(&self) {
///         // route implementation
///     }
/// }
/// ```
pub fn patch(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_str::<syn::Expr>(args.to_string().as_str()).unwrap();
    let args_with_method = quote::quote! { "PATCH", #args };
    route_macro(args_with_method.into(), input)
}

#[proc_macro_attribute]
/// The `head` attribute is used to mark a controller method as a HEAD route.
///
/// ##### Arguments
/// * `path` - The path that the route will handle.
///
/// ##### Example
/// ```rust ignore
/// #[controller]
/// struct MyController {
///     // controller fields and methods
/// }
///
/// #[routes]
/// impl MyController {
///     #[head("/users")]
///     fn get_users(&self) {
///         // route implementation
///     }
/// }
/// ```
pub fn head(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_str::<syn::Expr>(args.to_string().as_str()).unwrap();
    let args_with_method = quote::quote! { "HEAD", #args };
    route_macro(args_with_method.into(), input)
}

#[proc_macro_attribute]
/// The `check` macro is used to determine if a route should be executed.
///
/// ##### Example
/// ```rust ignore
/// #[check(CheckGate)]
/// fn my_route(&self) {
///     // route implementation
/// }
/// ```
pub fn check(args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_input = syn::parse::<syn::Item>(input.clone());
    match parsed_input {
        Ok(syn::Item::Fn(_)) => check_fn_macro(args, input),
        Ok(syn::Item::Impl(impl_item)) => check_impl_macro(impl_item, args),
        _ => panic!("`check` attribute can only be used on methods or impl blocks"),
    }
}

#[proc_macro_derive(Dto)]
/// The `Dto` derive macro is used to generate a DTO struct.
///
/// ##### Example
/// ```rust ignore
/// #[derive(Dto)]
/// struct MyDto {
///     // fields
/// }
/// ```
pub fn derive_dto(input: TokenStream) -> TokenStream {
    dto_macro(input)
}
