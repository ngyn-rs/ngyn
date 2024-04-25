use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, ItemImpl};

use crate::utils::str_to_ident;

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

pub fn check_impl_macro(impl_item: ItemImpl, args: TokenStream) -> TokenStream {
    let mut new_items = Vec::new();
    let attrs = syn::Attribute {
        pound_token: Default::default(),
        style: syn::AttrStyle::Outer,
        bracket_token: Default::default(),
        meta: syn::Meta::List(syn::MetaList {
            path: syn::Path::from(str_to_ident("check".to_string())),
            delimiter: syn::MacroDelimiter::Paren(Default::default()),
            tokens: args.into(),
        }),
    };

    for item in impl_item.items {
        let new_item = match item {
            syn::ImplItem::Fn(mut method) => {
                if method.attrs.clone().into_iter().any(|attr| {
                    attr.path().is_ident("route")
                        || attr.path().is_ident("get")
                        || attr.path().is_ident("post")
                        || attr.path().is_ident("put")
                        || attr.path().is_ident("delete")
                        || attr.path().is_ident("patch")
                        || attr.path().is_ident("head")
                        || attr.path().is_ident("options")
                }) {
                    method.attrs.push(attrs.clone());
                }
                syn::ImplItem::Fn(method)
            }
            _ => item,
        };
        new_items.push(new_item);
    }
    let new_impl = syn::Item::Impl(syn::ItemImpl {
        items: new_items,
        ..impl_item
    });
    quote::quote!(#new_impl).into()
}

pub fn check_fn_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(input as ItemFn);
    let CheckArgs { gates } = syn::parse_macro_input!(args as CheckArgs);

    let req = match input.sig.inputs.iter().nth(1) {
        Some(syn::FnArg::Typed(pat_type)) => &pat_type.pat,
        _ => panic!("Expected a valid request parameter"),
    };

    let res = match input.sig.inputs.iter().nth(2) {
        Some(syn::FnArg::Typed(pat_type)) => &pat_type.pat,
        _ => panic!("Expected a valid response parameter"),
    };

    let gates = gates.iter().map(|gate| {
        quote! {
            {
                use ngyn::prelude::NgynGate;
                let gate: #gate = ngyn::prelude::NgynProvider.provide();
                if !gate.can_activate(#req) {
                    #res.set_status(403);
                    return ngyn::prelude::Bytes::from("Forbidden".to_string()).parse_bytes();
                }
            }
        }
    });

    input.block.stmts.insert(
        0,
        syn::parse2::<syn::Stmt>(quote! {
            #(#gates)*
        })
        .unwrap(),
    );

    let check_fn = quote! {
        #input
    };

    check_fn.into()
}
