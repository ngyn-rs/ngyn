use proc_macro::TokenStream;
use quote::quote;

use crate::utils::parse_macro_data::parse_macro_data;

struct ControllerArgs {
    prefix: Option<syn::LitStr>,
    middlewares: Vec<syn::Path>,
    init: Option<syn::LitStr>,
    inject: Option<syn::LitStr>,
}

impl syn::parse::Parse for ControllerArgs {
    /// Parses a string like `prefix="/weather", middlewares = [WeatherGate]`
    /// into a `ControllerArgs` struct.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut middlewares = vec![];
        let mut prefix = None;
        let mut init = None;
        let mut inject = None;

        // match the input with format "/prefix"
        if !input.is_empty() && input.peek(syn::LitStr) {
            let path: syn::LitStr = input.parse()?;
            prefix = Some(path);

            return Ok(ControllerArgs {
                middlewares,
                prefix,
                init,
                inject,
            });
        }

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;

            match ident.to_string().as_str() {
                "prefix" => {
                    let path: syn::LitStr = input.parse()?;
                    prefix = Some(path);
                }
                "middlewares" => {
                    let content;
                    syn::bracketed!(content in input);
                    while !content.is_empty() {
                        let path: syn::Path = content.parse()?;
                        middlewares.push(path);
                        if !content.is_empty() {
                            content.parse::<syn::Token![,]>()?;
                        }
                    }
                }
                "init" => {
                    init = input.parse()?;
                }
                "inject" => {
                    inject = input.parse()?;
                }
                _ => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("unexpected attribute `{}`", ident),
                    ));
                }
            }
            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(ControllerArgs {
            middlewares,
            prefix,
            init,
            inject,
        })
    }
}

pub(crate) fn controller_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    let syn::DeriveInput {
        attrs,
        ident,
        data,
        vis,
        generics,
    } = syn::parse_macro_input!(input as syn::DeriveInput);
    let ControllerArgs {
        middlewares,
        prefix,
        init,
        inject,
    } = syn::parse_macro_input!(args as ControllerArgs);

    let generics_params = if generics.params.iter().count() > 0 {
        let generics_params = generics.params.iter().map(|param| {
            if let syn::GenericParam::Type(ty) = param {
                let ident = &ty.ident;
                quote! { #ident }
            } else {
                quote! { #param }
            }
        });
        quote! {
            <#(#generics_params),*>
        }
    } else {
        quote! {}
    };

    let controller_fields = parse_macro_data(data);

    let mut add_fields = Vec::new();
    let mut inject_fields = Vec::new();
    let fields: Vec<_> = controller_fields
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
                add_fields.push(quote! {
                    #ident #colon_token #ty::default()
                });
                if attrs.iter().any(|attr| attr.path().is_ident("inject")) {
                    inject_fields.push(quote! {
                        self.#ident.inject(cx);
                    });
                }
                let attrs = attrs.iter().filter(|attr| !attr.path().is_ident("inject"));
                quote! {
                    #(#attrs),* #vis #ident #colon_token #ty
                }
            },
        )
        .collect();

    let add_middlewares: Vec<_> = middlewares
        .iter()
        .map(|m| {
            quote! {
                let mut middleware = #m::default();
                middleware.inject(cx);
                middleware.handle(cx, res);
            }
        })
        .collect();

    let path_prefix = {
        if let Some(prefix) = prefix {
            let str_prefix = prefix.value();
            if !str_prefix.starts_with("/") {
                quote! {
                    format!("/{}", #prefix)
                }
            } else {
                quote! {
                    format!("{}", #prefix)
                }
            }
        } else {
            quote! {
                "".to_string()
            }
        }
    };

    let init_controller = {
        if let Some(init) = init {
            let init_ident = init.parse::<syn::Ident>().unwrap();
            quote! {
                #ident::#init_ident()
            }
        } else {
            quote! {
                #ident {
                    #(#add_fields),*
                }
            }
        }
    };

    let inject_controller = {
        if let Some(inject) = inject {
            let inject_ident = inject.parse::<syn::Ident>().unwrap();
            quote! {
                #ident::#inject_ident()
            }
        } else {
            quote! {}
        }
    };

    let expanded = quote! {
        #(#attrs)*
        #vis struct #ident #generics {
            #(#fields),*
        }

        impl #generics ngyn::shared::traits::NgynControllerHandler for #ident #generics_params {}

        impl #generics ngyn::shared::traits::NgynInjectable for #ident #generics_params {
            fn new() -> Self {
                #init_controller
            }

            fn inject(&mut self, cx: &ngyn::prelude::NgynContext) {
                #(#inject_fields)*
                #inject_controller
            }
        }

        #[ngyn::prelude::async_trait]
        impl #generics ngyn::shared::traits::NgynController for #ident #generics_params {
            fn routes(&self) -> Vec<(String, String, String)> {
                Self::ROUTES.iter().map(|(path, method, handler)| {
                    // prefix path with controller prefix, and remove double slashes
                    let path = format!("{}", path).trim_start_matches("/").to_string();
                    let prefix = #path_prefix.trim_end_matches("/").to_string();
                    (format!("{}/{}", prefix, path), method.to_string(), handler.to_string())
                }).collect()
            }

            fn prefix(&self) -> String {
                #path_prefix
            }

            async fn handle(
                &mut self,
                handler: &str,
                cx: &mut ngyn::prelude::NgynContext,
                res: &mut ngyn::prelude::NgynResponse,
            ) {
                #(#add_middlewares)*
                self.__handle_route(handler, cx, res).await;
            }
        }
    };
    expanded.into()
}
