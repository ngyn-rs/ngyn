use http::StatusCode;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn, LitInt};

pub(crate) fn http_code_macro(args: TokenStream, raw_input: TokenStream) -> TokenStream {
    match syn::parse::<ItemFn>(raw_input.clone()) {
        Ok(_) => {
            let args = parse_macro_input!(args as LitInt);
            let code = args.base10_parse::<u16>().unwrap();
            StatusCode::from_u16(code)
                .unwrap_or_else(|_| panic!("Invalid HTTP status code: {}", code));
        }
        Err(err) => {
            panic!("failed to parse route: {}", err)
        }
    };

    raw_input
}
