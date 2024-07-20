use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(SwaggerDto)]
pub fn swagger_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match input.data {
        Data::Struct(d) => d.fields,
        _ => unimplemented!(),
    };

    let mut fields_list = Vec::new();
    let mut required_fields = Vec::new();
    for field in fields {
        if let Some(ident) = field.ident {
            let ty_type = match field.ty {
                syn::Type::Path(ty) => ty.path.get_ident().unwrap().to_string(),
                syn::Type::Array(ty) => {
                    let ty = match *ty.elem {
                        syn::Type::Path(ty) => ty.path.segments.first().unwrap().ident.to_string(),
                        _ => unimplemented!(),
                    };
                    format!("Vec<{}>", ty)
                }
                syn::Type::Reference(ty) => {
                    let ty = match *ty.elem {
                        syn::Type::Path(ty) => ty.path.get_ident().unwrap().to_string(),
                        _ => unimplemented!(),
                    };
                    ty
                }
                _ => unimplemented!(),
            };
            let str_quote_ident = quote! { stringify!(#ident) };

            if !ty_type.starts_with("Option<") {
                required_fields.push(str_quote_ident.clone());
            }
            let value_type = match ty_type.as_str() {
                "String" => "string",
                "i32" | "i64" | "u32" | "u64" => "integer",
                "f32" | "f64" => "number",
                "bool" => "boolean",
                ty => {
                    if ty.starts_with("Vec<") {
                        "array"
                    } else if ty.starts_with("Option<") {
                        let ty = ty.trim_start_matches("Option<").trim_end_matches(">");
                        if ty == "String" {
                            "string"
                        } else if ty == "i32" || ty == "i64" || ty == "u32" || ty == "u64" {
                            "integer"
                        } else if ty == "f32" || ty == "f64" {
                            "number"
                        } else if ty == "bool" {
                            "boolean"
                        } else {
                            "object"
                        }
                    } else {
                        "object"
                    }
                }
            };
            let format_type = match ty_type.as_str() {
                "i32" | "u32" => "int32",
                "i64" | "u64" => "int64",
                "f32" => "float",
                "f64" => "double",
                _ => "",
            };

            if format_type.is_empty() {
                fields_list.push(quote! {
                    #str_quote_ident: {
                        "type": #value_type
                    }
                });
                continue;
            }
            fields_list.push(quote! {
                #str_quote_ident: {
                    "type": #value_type,
                    "format": #format_type
                }
            });
        }
    }

    let str_quote_name = quote! { stringify!(#name) };

    let expanded = quote! {
        impl ngyn_swagger::SwaggerDto for #name {
            fn to_swagger() -> serde_json::Value {
                serde_json::json!({
                    #str_quote_name: {
                        "type": "object",
                        "required": [#(#required_fields),*],
                        "properties": {
                            #(#fields_list),*
                        }
                    }
                })
            }
        }
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn swagger_attribute(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = quote! {
        #input

        impl ngyn_swagger::SwaggerDto for #input {
            fn to_swagger() -> serde_json::Value {
                serde_json::json!({
                    "components": []
                })
            }
        }
    };
    expanded.into()
}
