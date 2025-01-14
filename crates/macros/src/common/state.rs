use proc_macro::TokenStream;

pub(crate) fn derive_app_state_macro(input: TokenStream) -> TokenStream {
    let syn::ItemStruct {
        ident, generics, ..
    } = syn::parse_macro_input!(input as syn::ItemStruct);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote::quote! {
        impl #impl_generics ngyn::shared::server::context::AppState for #ident #ty_generics #where_clause {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }

        impl<'a> #impl_generics ngyn::shared::server::Transformer<'a> for &'a #ident #ty_generics #where_clause {
            fn transform(cx: &'a mut ngyn::prelude::NgynContext<'_>) -> Self {
                cx.state::<#ident>().unwrap()
            }
        }
        impl<'a> #impl_generics ngyn::shared::server::Transformer<'a> for &'a mut #ident #ty_generics #where_clause {
            fn transform(cx: &'a mut ngyn::prelude::NgynContext<'_>) -> Self {
                cx.state_mut::<#ident>().unwrap()
            }
        }
    };
    TokenStream::from(expanded)
}
