use proc_macro::TokenStream;

pub(crate) fn inject_macro(_args: TokenStream, raw_input: TokenStream) -> TokenStream {
    raw_input
}
