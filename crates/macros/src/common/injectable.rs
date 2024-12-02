use proc_macro::TokenStream;
use quote::quote;

struct InjectableArgs {
    init: Option<syn::LitStr>,
    inject: Option<syn::LitStr>,
}

impl syn::parse::Parse for InjectableArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut init = None;
        let mut inject = None;

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;

            match ident.to_string().as_str() {
                "init" => {
                    init = input.parse()?;
                }
                "inject" => {
                    inject = input.parse()?;
                }
                _ => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("unexpected argument `{}`", ident),
                    ));
                }
            }
        }

        Ok(InjectableArgs { init, inject })
    }
}

pub(crate) fn injectable_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    let InjectableArgs { init, inject } = syn::parse_macro_input!(args as InjectableArgs);
    let syn::ItemStruct {
        attrs,
        vis,
        ident,
        generics,
        fields,
        struct_token,
        ..
    } = syn::parse_macro_input!(input as syn::ItemStruct);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut add_fields = Vec::new();
    let mut inject_fields = Vec::new();
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

    let inject_injectable = match inject {
        Some(inject) => {
            let inject_ident = inject.parse::<syn::Ident>().unwrap();
            quote! {
                #ident::#inject_ident(cx)
            }
        }
        None => quote! {},
    };

    let expanded = quote! {
        #(#attrs)*
        #vis #struct_token #ident #generics {
            #(#fields),*
        }

        impl #impl_generics ngyn::prelude::NgynInjectable for #ident #ty_generics #where_clause {
            fn new() -> Self {
                #init_injectable
            }

            fn inject(&mut self, cx: &ngyn::prelude::NgynContext) {
                #(#inject_fields)*
                #inject_injectable
            }
        }

        impl #impl_generics ngyn::shared::server::Transformer<'_> for #ident #ty_generics #where_clause {
            fn transform(cx: &mut NgynContext) -> Self {
                let mut item = Self::default();
                item.inject(cx);
                item
            }
        }

        impl #impl_generics Default for #ident #ty_generics #where_clause {
            fn default() -> Self {
                Self::new()
            }
        }
    };
    expanded.into()
}
