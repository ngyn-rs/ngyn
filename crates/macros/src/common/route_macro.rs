use proc_macro::TokenStream;
use syn::ItemFn;

pub fn route_macro(_args: TokenStream, raw_input: TokenStream) -> TokenStream {
    match syn::parse::<ItemFn>(raw_input.clone()) {
        Ok(input) => input,
        Err(err) => {
            panic!("failed to parse route: {}", err)
        }
    };

    raw_input
}
