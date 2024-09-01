use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub(crate) fn param_macro(input: TokenStream) -> TokenStream {
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

    let field_names: Vec<_> = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap().clone())
        .collect();

    let expanded = quote! {
        impl #generics ngyn::shared::server::Transformer<'_> for #ident #generics {
            fn transform(cx: &mut ngyn::prelude::NgynContext, res: &mut ngyn::prelude::NgynResponse) -> Self {
                let param = ngyn::shared::server::Param::transform(cx, res);
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
