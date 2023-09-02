use syn::Ident;

pub fn str_to_ident(string: String) -> Ident {
    Ident::new(&string, proc_macro::Span::call_site().into())
}
