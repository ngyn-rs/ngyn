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
    };
    TokenStream::from(expanded)
}
