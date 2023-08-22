use proc_macro::TokenStream;
use quote::quote;
use rustle_shared::path_enum::Path;
use syn::ItemFn;

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

    let _path = args.path.unwrap();

    let ident = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let output = match &input.sig.output {
        syn::ReturnType::Default => panic!("expected a valid return type"),
        syn::ReturnType::Type(_, ty) => ty,
    };
    let block = &input.block;

    let expanded = quote! {
        pub fn #ident(#inputs) -> #output {
            #block
        }
    };

    expanded.into()
}
