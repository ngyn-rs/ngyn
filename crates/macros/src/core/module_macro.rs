use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::utils::parse_macro_data::parse_macro_data;

struct ModuleArgs {
    imports: Vec<syn::Path>,
    controllers: Vec<syn::Path>,
    init: Option<syn::LitStr>,
}

impl syn::parse::Parse for ModuleArgs {
    /// Parses the attribute arguments of a `#[module]` macro.
    /// We make sure that the arguments are in the format `controllers = [], imports = [], init = "new"`.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut imports = vec![];
        let mut controllers = vec![];
        let mut init = None;

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;

            let content;
            syn::bracketed!(content in input);

            while !content.is_empty() {
                match ident.to_string().as_str() {
                    "imports" => imports.push(content.parse()?),
                    "controllers" => controllers.push(content.parse()?),
                    "init" => init = Some(content.parse()?),
                    _ => {
                        return Err(syn::Error::new(
                            ident.span(),
                            format!("unexpected attribute `{}`", ident),
                        ))
                    }
                }
                if !content.is_empty() {
                    content.parse::<syn::Token![,]>()?;
                }
            }

            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(ModuleArgs {
            imports,
            controllers,
            init,
        })
    }
}

pub(crate) fn module_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        attrs,
        vis,
        generics,
        data,
    } = parse_macro_input!(input as DeriveInput);
    let args = parse_macro_input!(args as ModuleArgs);
    let module_fields = parse_macro_data(data);

    let fields: Vec<_> = module_fields
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

    let add_fields: Vec<_> = module_fields
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

    let add_controllers: Vec<_> = args
        .controllers
        .iter()
        .map(|controller| {
            quote! {
                let controller: #controller = #controller::new();
                controllers.push(std::sync::Arc::new(controller));
            }
        })
        .collect();

    let add_imported_modules_controllers: Vec<_> = args
        .imports
        .iter()
        .map(|import| {
            quote! {
                let mut module: #import = #import::new();
                module.get_controllers().iter().for_each(|controller| {
                    controllers.push(controller.clone());
                });
            }
        })
        .collect();

    let init_module = match args.init {
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

        impl #generics ngyn::shared::traits::NgynModule for #ident #generics {
            fn new() -> Self {
                #init_module
            }
            fn name(&self) -> &str {
                stringify!(#ident)
            }
            fn get_controllers(&mut self) -> Vec<std::sync::Arc<dyn ngyn::shared::traits::NgynController>> {
                use ngyn::shared::traits::NgynController;
                let mut modules: Vec<std::sync::Arc<dyn ngyn::shared::traits::NgynModule>> = vec![];
                let mut controllers: Vec<std::sync::Arc<dyn ngyn::shared::traits::NgynController>> = vec![];
                #(#add_controllers)*
                #(#add_imported_modules_controllers)*
                controllers
            }
        }
    };
    TokenStream::from(expanded)
}
