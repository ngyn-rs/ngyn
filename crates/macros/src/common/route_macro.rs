use ngyn_shared::path_enum::Path;
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

use crate::utils::str_to_ident;

struct Args {
    path: Option<Path>,
    http_method: String,
}

pub fn route_macro(args: TokenStream, raw_input: TokenStream) -> TokenStream {
    let input = match syn::parse::<ItemFn>(raw_input) {
        Ok(input) => input,
        Err(err) => {
            let error_message = err.to_string();
            panic!("failed to parse method: {}", error_message)
        }
    };

    let args = {
        let input_str = args.to_string();

        let (path, http_method) = if input_str.starts_with('"') {
            // the input_str should be of this structure: `"GET", "/path"` or `"GET", ["/path1", "/path2"]`
            // match the first argument to see if it's a valid HTTP method
            let mut input_str = input_str.trim_matches('"').split(',');
            let http_method = input_str
                .next()
                .unwrap()
                .to_string()
                .trim_matches('"')
                .to_uppercase();

            // panic if the HTTP method is invalid
            if !["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"]
                .contains(&http_method.as_str())
            {
                panic!("invalid HTTP method: {}", http_method)
            }

            // match everything else as the path or paths
            let route_path = input_str
                .skip(0)
                .map(|s| s.trim())
                .collect::<Vec<_>>()
                .join(",");
            let path = if route_path.starts_with('[') {
                let paths: Vec<_> = route_path
                    .trim_matches(|p| p == '[' || p == ']')
                    .split(',')
                    .map(|p| p.trim_matches('"').to_string())
                    .collect();
                Some(Path::Multiple(paths))
            } else if route_path.is_empty() {
                Some(Path::Single("/".to_string()))
            } else {
                Some(Path::Single(route_path.trim_matches('"').to_string()))
            };
            (path, http_method)
        } else {
            panic!("route expects at least one argument")
        };

        Args { path, http_method }
    };

    let path = args.path.unwrap();
    let http_method = args.http_method;

    let ident = &input.sig.ident;
    let fn_vis = &input.vis;
    let fn_async = &input.sig.asyncness;
    let inputs = &input.sig.inputs;
    let output = match &input.sig.output {
        syn::ReturnType::Default => quote! { ngyn::NgynResponse },
        syn::ReturnType::Type(_, ty) => quote! { #ty },
    };
    let block = &input.block;

    let mut expanded_methods: Vec<_> = Vec::new();

    match path {
        Path::Multiple(paths) => {
            for path in paths {
                let route_code = quote! {
                    self.routes.push((#path.to_string(), #http_method.to_string(), stringify!(#ident).to_string()));
                };
                expanded_methods.push(route_code);
            }
        }
        Path::Single(path) => {
            let route_code = quote! {
                self.routes.push((#path.to_string(), #http_method.to_string(), stringify!(#ident).to_string()));
            };
            expanded_methods.push(route_code);
        }
    }

    let register_ident = str_to_ident(format!("register_{}", ident));

    let expanded = quote! {
        #fn_vis #fn_async fn #ident(#inputs) -> #output #block

        fn #register_ident(&mut self) {
            #(#expanded_methods)*
        }
    };

    expanded.into()
}
