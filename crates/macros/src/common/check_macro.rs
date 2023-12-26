use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, Signature};

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

    let Signature {
        ident,
        inputs,
        output,
        fn_token,
        ..
    } = sig;

    let req = match inputs.iter().nth(1) {
        Some(syn::FnArg::Typed(pat_type)) => &pat_type.pat,
        _ => panic!("Expected a valid request parameter"),
    };

    let res = match inputs.iter().nth(2) {
        Some(syn::FnArg::Typed(pat_type)) => &pat_type.pat,
        _ => panic!("Expected a valid response parameter"),
    };

    let gates = gates.iter().map(|gate| {
        quote! {
            {
                use ngyn::NgynGate;
                let gate: #gate = ngyn::NgynProvider.provide();
                if !gate.can_activate(#req) {
                    #res.set_status(403);
                    return ngyn::NgynBody::String("Forbidden".to_string());
                }
            }
        }
    });

    let check_fn = quote! {
        #(#attrs)*
        #vis async #fn_token #ident(#inputs) #output {
            #(#gates)*
            #block
        }
    };

    check_fn.into()
}
