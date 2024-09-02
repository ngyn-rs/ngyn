use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub(crate) fn query_macro(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = syn::parse_macro_input!(input as DeriveInput);

    let fields = match data {
        Data::Struct(data_struct) => data_struct.fields,
        _ => panic!("This macro only supports structs."),
    };

    let field_names: Vec<_> = fields.iter().map(|field| field.ident.as_ref()).collect();

    let expanded = quote! {
        impl #generics ngyn::shared::server::Transformer<'_> for #ident #generics {
            fn transform(cx: &mut ngyn::prelude::NgynContext, res: &mut ngyn::prelude::NgynResponse) -> Self {
                let query = ngyn::shared::server::Query::transform(cx, res);
                #ident {
                    #(
                        #field_names: query.get(stringify!(#field_names)).unwrap_or_default(),
                    )*
                }
            }
        }
    };

    expanded.into()
}
