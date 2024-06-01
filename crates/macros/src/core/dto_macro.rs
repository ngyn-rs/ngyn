use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::utils::parse_macro_data;

struct DtoMacroArgs {
    validator: Option<syn::LitStr>,
    reporter: Option<syn::LitStr>,
}

impl syn::parse::Parse for DtoMacroArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut validator = None;
        let mut reporter = None;
        while !input.is_empty() {
            let ident = input.parse::<syn::Ident>()?;
            match ident.to_string().as_str() {
                "validator" => {
                    let _: syn::Token![=] = input.parse()?;
                    validator = Some(input.parse()?);
                }
                "reporter" => {
                    let _: syn::Token![=] = input.parse()?;
                    reporter = Some(input.parse()?);
                }
                _ => {
                    return Err(syn::Error::new(
                        ident.span(),
                        "Unknown attribute for dto macro",
                    ));
                }
            }
        }
        return Ok(Self {
            validator,
            reporter,
        });
    }
}

pub fn dto_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        attrs,
        vis,
        generics,
    } = syn::parse_macro_input!(input as DeriveInput);
    let DtoMacroArgs {
        validator,
        reporter,
    } = syn::parse_macro_input!(args as DtoMacroArgs);

    let (types, keys) = parse_macro_data(data);

    let fields: Vec<_> = types
        .iter()
        .zip(keys.iter())
        .map(|(ty, key)| {
            quote! {
                #key: #ty
            }
        })
        .collect();

    let validation = if validator.is_some() {
        let ident = validator.unwrap().parse::<syn::Ident>().unwrap();
        let reporter = if reporter.is_some() {
            let ident = reporter.unwrap().parse::<syn::Ident>().unwrap();
            quote! {
                res.send(#ident(err));
            }
        } else {
            quote! {
                res.send(err.to_string());
            }
        };
        quote! {
            if let Err(err) = data.#ident() {
                res.set_status(400);
                #reporter
                return None;
            }
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        #(#attrs)*
        #vis struct #ident #generics {
            #(#fields),*
        }

        impl ngyn::prelude::Transformer for #ident {
            fn transform(cx: &mut ngyn::prelude::NgynContext, res: &mut ngyn::prelude::NgynResponse) -> Option<Self> {
                let dto = ngyn::prelude::Dto::transform(cx, res).unwrap();
                match dto.parse::<#ident>() {
                    Ok(data) => {
                        #validation
                        return Some(data);
                    }
                    Err(err) => {
                        res.set_status(400);
                        res.send(err.to_string());
                        return None;
                    }
                }
            }
        }
    };
    expanded.into()
}
