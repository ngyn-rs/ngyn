extern crate proc_macro;

mod common;
mod core;
mod utils;

use crate::common::check_macro::check_fn_macro;
use crate::common::{controller_macro::*, injectable_macro::*, route_macro::*, routes_macro::*};
use crate::core::dto_macro::dto_macro;
use crate::core::module_macro::*;
use crate::core::platform_macro::platform_macro;

use common::check_macro::check_impl_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
/// `platform` is used to mark a struct as a platform engine.
pub fn platform(args: TokenStream, input: TokenStream) -> TokenStream {
    platform_macro(args, input)
}

#[proc_macro_attribute]
/// `module` is a procedural macro that generates a struct and its implementation.
pub fn module(args: TokenStream, input: TokenStream) -> TokenStream {
    module_macro(args, input)
}

#[proc_macro_attribute]
/// The `injectable` attribute is used to mark a struct as injectable.
/// This means that the struct can be automatically provided as a dependency where needed.
pub fn injectable(args: TokenStream, input: TokenStream) -> TokenStream {
    injectable_macro(args, input)
}

#[proc_macro_attribute]
/// The `controller` attribute is used to mark a struct as a controller.
pub fn controller(args: TokenStream, input: TokenStream) -> TokenStream {
    controller_macro(args, input)
}

#[proc_macro_attribute]
/// The `routes` attribute is used to mark a struct impl as a routes container.
pub fn routes(_args: TokenStream, input: TokenStream) -> TokenStream {
    routes_macro(input)
}

#[proc_macro_attribute]
/// The `route` attribute is used to mark a function as a route.
///
/// ##### Arguments
/// * `method` - The HTTP method that the route will handle. (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS)
/// * `path` - The path that the route will handle.
pub fn route(args: TokenStream, input: TokenStream) -> TokenStream {
    route_macro(args, input)
}

#[proc_macro_attribute]
/// The `get` attribute is used to mark a function as a GET route.
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_str::<syn::Expr>(args.to_string().as_str()).unwrap();
    let args_with_method = quote::quote! { "GET", #args };
    route_macro(args_with_method.into(), input)
}

#[proc_macro_attribute]
/// The `post` attribute is used to mark a function as a GET route.
pub fn post(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_str::<syn::Expr>(args.to_string().as_str()).unwrap();
    let args_with_method = quote::quote! { "POST", #args };
    route_macro(args_with_method.into(), input)
}

#[proc_macro_attribute]
/// The `put` attribute is used to mark a function as a GET route.
pub fn put(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_str::<syn::Expr>(args.to_string().as_str()).unwrap();
    let args_with_method = quote::quote! { "PUT", #args };
    route_macro(args_with_method.into(), input)
}

#[proc_macro_attribute]
/// The `delete` attribute is used to mark a function as a GET route.
pub fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_str::<syn::Expr>(args.to_string().as_str()).unwrap();
    let args_with_method = quote::quote! { "DELETE", #args };
    route_macro(args_with_method.into(), input)
}

#[proc_macro_attribute]
/// The `patch` attribute is used to mark a function as a GET route.
pub fn patch(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_str::<syn::Expr>(args.to_string().as_str()).unwrap();
    let args_with_method = quote::quote! { "PATCH", #args };
    route_macro(args_with_method.into(), input)
}

#[proc_macro_attribute]
/// The `head` attribute is used to mark a function as a GET route.
pub fn head(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_str::<syn::Expr>(args.to_string().as_str()).unwrap();
    let args_with_method = quote::quote! { "HEAD", #args };
    route_macro(args_with_method.into(), input)
}

#[proc_macro_attribute]
/// `check` macro is used to determine if a route should be executed.
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
pub fn dto(input: TokenStream) -> TokenStream {
    dto_macro(input)
}
