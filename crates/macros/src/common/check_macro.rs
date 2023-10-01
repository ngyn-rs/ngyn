use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

struct CheckArgs {
    gates: Vec<syn::Path>,
}

impl syn::parse::Parse for CheckArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut gates = vec![];
        while !input.is_empty() {
            let path: syn::Path = input.parse()?;
            gates.push(path);
            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }
        if gates.is_empty() {
            return Err(syn::Error::new(input.span(), "expected at least one gate"));
        }
        Ok(Self { gates })
    }
}

pub fn check_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    let CheckArgs { gates } = syn::parse_macro_input!(args as CheckArgs);
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = syn::parse_macro_input!(input as ItemFn);

    let gates = gates.iter().map(|gate| {
        quote! {
            let gate: #gate = ngyn::NgynProvider.provide();
            if !gate.check(req) {
                return;
            }
        }
    });

    let check_fn = quote! {
        #(#attrs)*
        #vis #sig {
            #(#gates)*
            #block
        }
    };

    check_fn.into()
}
