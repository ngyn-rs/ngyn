use proc_macro::TokenStream;
use quote::quote;
use rustle_shared::path_enum::Path;
use syn::ItemFn;

use crate::utils::str_to_ident;

struct Args {
    path: Option<Path>,
}

pub fn route_get_macro(args: TokenStream, raw_input: TokenStream) -> TokenStream {
    let input = match syn::parse::<ItemFn>(raw_input) {
        Ok(input) => input,
        Err(err) => {
            let error_message = err.to_string();
            panic!("failed to parse input: {}", error_message)
        }
    };

    let args = {
        let input_str = args.to_string();

        let path = if input_str.starts_with("\"") && input_str.ends_with("\"") {
            Some(Path::Single(
                input_str.trim_matches('"').to_string().to_lowercase(),
            ))
        } else if input_str.starts_with('[') && input_str.ends_with(']') {
            let paths: Vec<String> = input_str[1..input_str.len() - 1]
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            Some(Path::Multiple(paths))
        } else if input_str.is_empty() {
            Some(Path::Single(input_str.to_string()))
        } else {
            panic!("invalid path")
        };

        Args { path }
    };

    let path = args.path.unwrap();

    let ident = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let output = match &input.sig.output {
        syn::ReturnType::Default => panic!("expected a valid return type"),
        syn::ReturnType::Type(_, ty) => ty,
    };
    let block = &input.block;
    let http_method = String::from("GET");

    let mut expanded_methods: Vec<_> = Vec::new();

    match path {
        Path::Multiple(paths) => {
            for path in paths {
                let route_code = quote! {
                    self.add_route(#path.to_string(), #http_method.to_string(), Box::new(move |req: rustle_core::RustleRequest, res: rustle_core::RustleResponse| -> rustle_core::RustleResponse {
                    res
                }));
                };
                expanded_methods.push(route_code);
            }
        }
        Path::Single(path) => {
            let route_code = quote! {
                self.add_route(#path.to_string(), #http_method.to_string(), Box::new(move |req: rustle_core::RustleRequest, res: rustle_core::RustleResponse| -> rustle_core::RustleResponse {
                    res
                }));
            };
            expanded_methods.push(route_code);
        }
    }

    let register_ident = str_to_ident(format!("register_{}", ident));

    let expanded = quote! {
        fn #ident(#inputs) -> #output {
            #block
        }

        fn #register_ident(&mut self) {
            #(#expanded_methods)*
        }
    };

    expanded.into()
}
