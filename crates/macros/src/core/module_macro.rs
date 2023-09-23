extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

struct ModuleArgs {
    imports: Vec<syn::Path>,
    controllers: Vec<syn::Path>,
    middlewares: Vec<syn::Path>,
}

impl syn::parse::Parse for ModuleArgs {
    /// Parses the attribute arguments of a `#[module]` macro.
    /// We make sure that the arguments are in the format `controllers = [], imports = []`.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut imports = vec![];
        let mut controllers = vec![];
        let mut middlewares = vec![];

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;

            let content;
            syn::bracketed!(content in input);

            while !content.is_empty() {
                let path: syn::Path = content.parse()?;
                match ident.to_string().as_str() {
                    "imports" => imports.push(path),
                    "controllers" => controllers.push(path),
                    "middlewares" => middlewares.push(path),
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
            middlewares,
        })
    }
}

pub fn module_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let args = parse_macro_input!(args as ModuleArgs);

    let ident = input.ident;

    let add_middlewares: Vec<_> = args
        .middlewares
        .iter()
        .map(|middleware| {
            quote! {
                let middleware: #middleware = ngyn::NgynProvider.provide();
                middlewares.push(std::sync::Arc::new(middleware));
            }
        })
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
                let module: #import = #import::new();
                module.get_controllers().iter().for_each(|controller| {
                    controllers.push(controller.clone());
                });
            }
        })
        .collect();

    let expanded = quote! {
        #[ngyn::dependency]
        pub struct #ident {}

        impl ngyn::NgynModule for #ident {

            fn new() -> Self {
                Self {}
            }

            fn name(&self) -> &str {
                stringify!(#ident)
            }

            fn get_controllers(&self) -> Vec<std::sync::Arc<dyn ngyn::NgynController>> {
                let mut modules: Vec<std::sync::Arc<dyn ngyn::NgynModule>> = vec![];
                let mut controllers: Vec<std::sync::Arc<dyn ngyn::NgynController>> = vec![];
                #(#add_controllers)*
                #(#add_imported_modules_controllers)*
                controllers
            }

            fn get_middlewares(&self) -> Vec<std::sync::Arc<dyn ngyn::NgynMiddleware>> {
                let mut middlewares:  Vec<std::sync::Arc<dyn ngyn::NgynMiddleware>> = vec![];
                #(#add_middlewares)*
                middlewares
            }
        }
    };
    TokenStream::from(expanded)
}
