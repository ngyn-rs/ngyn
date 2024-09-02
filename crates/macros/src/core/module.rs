use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

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
                            format!("unexpected argument `{}`", ident),
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

    let fields = match data {
        Data::Struct(data_struct) => data_struct.fields,
        _ => panic!("This macro only supports structs."),
    };

    let mut add_fields = Vec::new();
    let fields: Vec<_> = fields
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
                    #ident #colon_token Default::default()
                });
                let attrs = attrs.iter().filter(|attr| !attr.path().is_ident("inject"));
                quote! {
                    #(#attrs),* #vis #ident #colon_token #ty
                }
            },
        )
        .collect();

    let add_controllers: Vec<_> = args
        .controllers
        .iter()
        .map(|controller| {
            quote! {
                std::sync::Arc::new(Box::new(#controller::new()) as Box<dyn ngyn::shared::traits::NgynController + 'static>)
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
            fn get_controllers(&self) -> Vec<std::sync::Arc<Box<dyn ngyn::shared::traits::NgynController + 'static>>> {
                use ngyn::shared::traits::NgynInjectable;
                let mut controllers: Vec<std::sync::Arc<Box<dyn ngyn::shared::traits::NgynController + 'static>>> = vec![#(#add_controllers),*];
                #(#add_imported_modules_controllers)*
                controllers
            }
        }
    };
    TokenStream::from(expanded)
}
