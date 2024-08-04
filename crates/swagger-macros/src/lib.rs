use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, ItemImpl, ReturnType, Signature};

#[proc_macro_derive(SwaggerComponent)]
pub fn swagger_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match input.data {
        Data::Struct(d) => d.fields,
        Data::Enum(d) => d.variants.iter().next().unwrap().fields.clone(),
        _ => unimplemented!(),
    };

    let mut fields_list = Vec::new();
    let mut required_fields = Vec::new();
    for field in fields {
        if let Some(ident) = field.ident {
            let ty_type = match field.ty {
                syn::Type::Path(ty) => {
                    if let Some(ident) = ty.path.get_ident() {
                        ident.to_string()
                    } else if let Some(segment) = ty.path.segments.last() {
                        segment.arguments.to_token_stream().to_string()
                    } else {
                        unimplemented!()
                    }
                }
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
                        let ty = ty.trim_start_matches("Option<").trim_end_matches('>');
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
        impl ngyn_swagger::SwaggerComponent for #name {
            fn to_swagger_schema(name: &str) -> serde_json::Value {
                serde_json::json!({
                    "name": name,
                    "in": "body",
                    "required": false,
                    "schema": {
                        "type": "object",
                        "required": [#(#required_fields),*],
                        "properties": {
                            #(#fields_list),*
                        }
                    }
                })
            }
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
pub fn swagger_route(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = match syn::parse::<ItemImpl>(input) {
        Ok(input) => input,
        Err(_err) => panic!("Only impl blocks are supported"),
    };

    let ItemImpl {
        items,
        impl_token,
        generics,
        self_ty,
        ..
    } = input.clone();

    let mut operations = None;
    let mut response = quote! {
        {
            "200": {
                "description": "Successful response"
            }
        }
    };

    let components = items.iter().fold(None, |components, item| match item {
        syn::ImplItem::Fn(method) => {
            let Signature {
                inputs,
                output,
                ident,
                ..
            } = method.sig.clone();
            let mut operation_meta = None;
            let mut retrieved_paths = Vec::new();
            let args = inputs.iter().fold(components, |args, input| match input {
                syn::FnArg::Typed(pat) => {
                    let ty = &pat.ty;
                    let pat = &pat.pat;
                    if let syn::Type::Reference(ty) = ty.as_ref() {
                        let ty = &ty.elem;
                        if let syn::Type::Path(ty) = ty.as_ref() {
                            if let Some(ty_ident) = ty.path.get_ident() {
                                let ty_str = ty_ident.to_string();
                                if !retrieved_paths.contains(&ty_str) {
                                    retrieved_paths.push(ty_str);
                                    if operation_meta.is_some() {
                                        operation_meta = Some(quote! {
                                            #operation_meta, #ty_ident::to_swagger_schema(stringify!(#pat))
                                        });
                                    } else {
                                        operation_meta = Some(quote! {
                                            #ty_ident::to_swagger_schema(stringify!(#pat))
                                        });
                                    }
                                    if args.is_some() {
                                        Some(quote! { #args, #ty_ident::to_swagger() })
                                    } else {
                                        Some(quote! { #ty_ident::to_swagger() })
                                    }
                                } else {
                                    args
                                }
                            } else {
                                args
                            }
                        } else {
                            args
                        }
                    } else if let syn::Type::Path(ty) = ty.as_ref() {
                        if let Some(ty_ident) = ty.path.get_ident() {
                            let ty_str = ty_ident.to_string();
                            if !retrieved_paths.contains(&ty_str) {
                                retrieved_paths.push(ty_str);
                                if operation_meta.is_some() {
                                    operation_meta = Some(quote! {
                                        #operation_meta, #ty_ident::to_swagger_schema(stringify!(#pat))
                                    });
                                } else {
                                    operation_meta = Some(quote! {
                                        #ty_ident::to_swagger_schema(stringify!(#pat))
                                    });
                                }
                                if args.is_some() {
                                    Some(quote! { #args, #ty_ident::to_swagger() })
                                } else {
                                    Some(quote! { #ty_ident::to_swagger() })
                                }
                            } else {
                                args
                            }
                        } else {
                            args
                        }
                    } else {
                        args
                    }
                }
                _ => args,
            });
            if let Some(operation_meta) = operation_meta {
                let next_operation = quote! { (stringify!(#ident).to_string(), vec![#operation_meta]) };
                if operations.is_some() {
                    operations = Some(quote! { #operations, #next_operation });
                } else {
                    operations = Some(next_operation);
                }
            }
            if let ReturnType::Type(_, ty) = output {
                if let syn::Type::Path(ty) = ty.as_ref() {
                    let ty_ident = ty.path.get_ident().unwrap();
                    let ty_str = ty_ident.to_string();
                    response = quote! {
                        {
                            "200": {
                                "description": "Successful response",
                                "content": {
                                    "application/json": {
                                        "schema": ""
                                    }
                                }
                            }
                        }
                    };
                    if !retrieved_paths.contains(&ty_str) {
                        retrieved_paths.push(ty_str);
                        if args.is_some() {
                            Some(quote! { #args, #ty_ident::to_swagger() })
                        } else {
                            Some(quote! { #ty_ident::to_swagger() })
                        }
                    } else {
                        args
                    }
                } else {
                    args
                }
            } else {
                args
            }
        }
        _ => None,
    });

    let expanded = quote! {
        #input

        #impl_token #generics ngyn_swagger::SwaggerController for #self_ty {
            fn swagger_meta(&self) -> ngyn_swagger::SwaggerMeta {
                use ngyn_swagger::SwaggerComponent;
                ngyn_swagger::SwaggerMeta {
                    components: vec![#components],
                    operations: vec![#operations],
                    response: serde_json::json! { #response }
                }
            }
        }
    };
    expanded.into()
}
