use proc_macro::TokenStream;
use quote::quote;

use crate::utils::handle_macro::handle_macro;

fn extract_route(method: &syn::ImplItemFn) -> Option<String> {
    let route_attr = method.attrs.iter().find(|attr| {
        if let Ok(meta) = attr.parse_args() {
            if let syn::Meta::Path(path) = meta {
                return path.is_ident("get")
                    || path.is_ident("post")
                    || path.is_ident("put")
                    || path.is_ident("delete");
            }
        }
        false
    });

    if let Some(attr) = route_attr {
        if let Ok(route_meta) = attr.parse_args() {
            if let syn::Meta::Path(path) = route_meta {
                return Some(path.segments.last().unwrap().ident.to_string());
            }
        }
    }

    None
}

fn find_impl_block(input: &syn::DeriveInput) -> Option<syn::ItemImpl> {
    if let syn::Data::Struct(_) = input.data {
        for item in &input.attrs {
            if let Ok(meta) = item.parse_args::<syn::Meta>() {
                if meta.path().is_ident("controller") {
                    if let syn::Item::Impl(impl_block) = syn::Item::from(input.clone()) {
                        return Some(impl_block);
                    }
                }
            }
        }
    }
    None
}

pub fn controller_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let (ident, types, keys) = handle_macro(input.clone());

    let mut routes = Vec::new();

    if let Some(impl_block) = find_impl_block(&input) {
        for item in impl_block.items {
            if let syn::ImplItem::Fn(method) = item {
                if let Some(route) = extract_route(&method) {
                    routes.push(route);
                }
            }
        }
    }

    let fields: Vec<_> = types
        .iter()
        .zip(keys.iter())
        .map(|(ty, key)| {
            quote! {
                #[allow(dead_code)]
                #key: #ty
            }
        })
        .collect();

    let expanded = quote! {
        #[rustle_core::dependency]
        pub struct #ident {
            #(#fields),*
        }

        impl #ident {
            pub fn new() -> Self {
                #ident {
                    #(#keys: rustle_core::RustleProvider.provide()),*
                }
            }
        }

        impl rustle_core::RustleController for #ident {

            fn name(&self) -> &str {
                stringify!(#ident)
            }

            fn routes(&self) -> Vec<(
                &str,
                &str,
                Box<
                    dyn Fn(rustle_core::RustleRequest, rustle_core::RustleResponse) -> rustle_core::RustleResponse
                        + Send
                        + Sync,
                >,
            )> {
                // build a vec of all methods available to this controller
                let mut routes: Vec<(
                    &str,
                    &str,
                    Box<
                        dyn Fn(rustle_core::RustleRequest, rustle_core::RustleResponse) -> rustle_core::RustleResponse
                            + Send
                            + Sync,
                    >,
                )> = vec![];

                // add all routes to the vec
                #(
                    routes.push((
                        #routes,
                        "GET",
                        Box::new(move |req, res| {
                            #ident::#routes(self.clone(), req, res)
                        })
                    ));
                )*

                routes
            }
        }

        impl rustle_core::RustleControllerInit for #ident {
            fn new() -> Box<dyn rustle_core::RustleController> {
                Box::new(Self::new())
            }
        }
    };
    expanded.into()
}
