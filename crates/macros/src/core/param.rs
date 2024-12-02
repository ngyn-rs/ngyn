use proc_macro::TokenStream;
use quote::quote;

pub(crate) fn param_macro(input: TokenStream) -> TokenStream {
    let syn::ItemStruct {
        ident,
        generics,
        fields,
        ..
    } = syn::parse_macro_input!(input as syn::ItemStruct);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let field_names: Vec<_> = fields.iter().map(|field| field.ident.as_ref()).collect();

    let expanded = quote! {
        impl #impl_generics ngyn::shared::server::Transformer<'_> for #ident #ty_generics #where_clause {
            fn transform(cx: &mut ngyn::prelude::NgynContext) -> Self {
                let param = ngyn::shared::server::Param::transform(cx);
                #ident {
                    #(
                        #field_names: param.get(stringify!(#field_names)).unwrap_or_default(),
                    )*
                }
            }
        }
    };

    expanded.into()
}
