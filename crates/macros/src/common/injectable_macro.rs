use proc_macro::TokenStream;
use quote::quote;

use crate::utils::parse_macro_data::parse_macro_data;

struct InjectableArgs {
    init: Option<syn::LitStr>,
}

impl syn::parse::Parse for InjectableArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut init = None;

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;

            match ident.to_string().as_str() {
                "init" => {
                    init = input.parse()?;
                }
                _ => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("unexpected attribute `{}`", ident),
                    ));
                }
            }
        }

        Ok(InjectableArgs { init })
    }
}

pub(crate) fn injectable_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    let syn::DeriveInput {
        ident,
        data,
        attrs,
        vis,
        generics,
    } = syn::parse_macro_input!(input as syn::DeriveInput);
    let InjectableArgs { init } = syn::parse_macro_input!(args as InjectableArgs);
    let injectable_fields = parse_macro_data(data);

    let fields: Vec<_> = injectable_fields
        .iter()
        .map(
            |syn::Field {
                 ident,
                 ty,
                 vis,
                 attrs,
                 colon_token,
                 ..
             }| {
                quote! {
                    #(#attrs),* #vis #ident #colon_token #ty
                }
            },
        )
        .collect();

    let add_fields: Vec<_> = injectable_fields
        .iter()
        .map(
            |syn::Field {
                 ident,
                 ty,
                 colon_token,
                 ..
             }| {
                quote! {
                    #ident #colon_token #ty::default()
                }
            },
        )
        .collect();

    let init_injectable = match init {
        Some(init) => {
            let init_ident = init.parse::<syn::Ident>().unwrap();
            quote! {
                #ident::#init_ident()
            }
        }
        None => quote! {
            #ident {
                #(#add_fields),*
            }
        },
    };

    let expanded = quote! {
        #(#attrs)*
        #vis struct #ident #generics {
            #(#fields),*
        }

        impl #generics ngyn::prelude::NgynInjectable for #ident #generics {
            fn new() -> Self {
                #init_injectable
            }
        }

        impl #generics Default for #ident #generics {
            fn default() -> Self {
                Self::new()
            }
        }
    };
    expanded.into()
}
