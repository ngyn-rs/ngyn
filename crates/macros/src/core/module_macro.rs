extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

struct ModuleArgs {
    imports: Vec<syn::Path>,
    controllers: Vec<syn::Path>,
}

impl syn::parse::Parse for ModuleArgs {
    /// Parses the attribute arguments of a `#[module]` macro.
    /// We make sure that the arguments are in the format `controllers = [], imports = []`.
    /// If the arguments are not in this format, we return an error.
    /// Otherwise, we return a `ModuleArgs` struct with the parsed arguments.
    /// The attributes are parsed as a `syn::Path` and stored in a `Vec`.
    /// They may also be omitted.
    ///
    /// # Examples
    ///
    /// ```
    /// #[module(controllers = [SampleController, SampleService])]
    ///
    /// #[module(imports = [SampleModule])]
    ///
    /// #[module(controllers = [SampleController], imports = [SampleModule])]
    /// ```
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut imports = vec![];
        let mut controllers = vec![];

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;

            match ident.to_string().as_str() {
                "imports" => {
                    let content;
                    syn::bracketed!(content in input);
                    while !content.is_empty() {
                        let path: syn::Path = content.parse()?;
                        imports.push(path);
                        if !content.is_empty() {
                            content.parse::<syn::Token![,]>()?;
                        }
                    }
                }
                "controllers" => {
                    let content;
                    syn::bracketed!(content in input);
                    while !content.is_empty() {
                        let path: syn::Path = content.parse()?;
                        controllers.push(path);
                        if !content.is_empty() {
                            content.parse::<syn::Token![,]>()?;
                        }
                    }
                }
                _ => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("unexpected attribute `{}`", ident),
                    ))
                }
            }

            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(ModuleArgs {
            imports,
            controllers,
        })
    }
}

pub fn module_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let args = parse_macro_input!(args as ModuleArgs);

    let ident = input.ident;

    let add_modules: Vec<_> = args
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
                #(#add_modules)*
                controllers
            }
        }
    };
    TokenStream::from(expanded)
}
