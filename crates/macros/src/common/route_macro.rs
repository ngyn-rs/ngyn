use ngyn_shared::{path_enum::Path, HttpMethod};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Signature};

use crate::utils::str_to_ident;

struct Args {
    path: Option<Path>,
    http_method: String,
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let http_method = input.parse::<syn::LitStr>()?.value();

        if HttpMethod::from(&http_method) == HttpMethod::Unknown {
            panic!("Unsupported HTTP method: {}", http_method)
        } else {
            input.parse::<syn::Token![,]>()?;
        }

        let path = if input.peek(syn::LitStr) {
            let path = input.parse::<syn::LitStr>()?;
            Some(Path::Single(path.value()))
        } else if input.peek(syn::token::Bracket) {
            let content;
            syn::bracketed!(content in input);
            let mut paths = Vec::new();
            while !content.is_empty() {
                let path = content.parse::<syn::LitStr>()?;
                paths.push(path.value());
                if !content.is_empty() {
                    content.parse::<syn::Token![,]>()?;
                }
            }
            Some(Path::Multiple(paths))
        } else {
            None
        };

        Ok(Args { path, http_method })
    }
}

pub fn route_macro(args: TokenStream, raw_input: TokenStream) -> TokenStream {
    let ItemFn {
        sig,
        block,
        vis,
        attrs,
    } = match syn::parse::<ItemFn>(raw_input) {
        Ok(input) => input,
        Err(err) => {
            let error_message = err.to_string();
            panic!("failed to parse method: {}", error_message)
        }
    };
    let args = parse_macro_input!(args as Args);

    let path = args.path.unwrap();
    let http_method = args.http_method;

    let Signature {
        ident,
        inputs,
        output,
        fn_token,
        ..
    } = sig;

    let return_val = match output {
        syn::ReturnType::Type(_, _) => quote! {},
        _ => quote! { return ngyn::NgynBody::None; },
    };

    let output = match output {
        syn::ReturnType::Type(_, ty) => quote! { -> #ty }, // TODO: Handle other types aside NgynBody
        _ => quote! { -> ngyn::NgynBody },
    };

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

    let register_ident = str_to_ident(format!("__{}", ident));

    let expanded = quote! {
        #(#attrs)*
        #vis async #fn_token #register_ident(#inputs) #output {
            #block
            #return_val
        }

        fn #ident(&mut self) {
            #(#expanded_methods)*
        }
    };

    expanded.into()
}
