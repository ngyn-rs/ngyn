use proc_macro::TokenStream;
use syn::ItemImpl;

use crate::utils::str_to_ident;

/// This macro is used to add a check attribute to all methods in an impl block.
pub fn check_impl_macro(impl_item: ItemImpl, args: TokenStream) -> TokenStream {
    let mut new_items = Vec::new();

    // Create the check attribute
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

/// This macro is used to add a check attribute handler to a method.
pub fn check_fn_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
    input.into()
}
