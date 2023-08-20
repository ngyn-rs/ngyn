use proc_macro::TokenStream;
use quote::quote;
use rustle_shared::path_enum::Path;
use syn::{parse::Parse, parse_macro_input, Attribute, ItemFn, Meta};

struct Args {
    path: Option<Path>,
    guards: Vec<String>,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut path = None;
        let mut guards = vec![];

        while !input.is_empty() {
            let mut attr_vec: Vec<Attribute> = Attribute::parse_outer(input).unwrap();
            let attr = attr_vec.pop();

            match attr {
                None => {
                    let input_str = input.fork().to_string();
                    if input_str.starts_with("\"") && input_str.ends_with("\"") {
                        path = Some(Path::Single(
                            input_str.trim_matches('"').to_string().to_lowercase(),
                        ));
                    } else if input_str.starts_with('[') && input_str.ends_with(']') {
                        let paths: Vec<String> = input_str[1..input_str.len() - 1]
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect();
                        path = Some(Path::Multiple(paths));
                    } else {
                        panic!("expected an attribute or a string, found `{}`", input_str)
                    }
                }
                _ => {
                    let attr = attr_vec.pop().unwrap();
                    let meta = attr.meta;

                    match meta {
                        Meta::NameValue(nv) => {
                            if nv.path.is_ident("path") {
                                if let syn::Expr::Lit(expr_lit) = &nv.value {
                                    if let syn::Lit::Str(lit) = &expr_lit.lit {
                                        let lit_value = lit.value();
                                        path = Some(Path::Single(lit_value));
                                    }
                                }
                            } else if nv.path.is_ident("guards") {
                                if let syn::Expr::Lit(expr_lit) = &nv.value {
                                    if let syn::Lit::Str(lit) = &expr_lit.lit {
                                        guards.push(lit.value());
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            };
        }

        Ok(Args { path, guards })
    }
}

pub fn route_get_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(args as Args);

    let path = args.path;
    let mut guards = args.guards;

    let ident = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;

    let expanded = quote! {
        pub fn #ident(#inputs) -> #output {
            #input
        }
    };

    expanded.into()
}
