use syn::Ident;

/// Converts a string into an Ident.
///
/// #### Arguments
///
/// * `string` - The string to convert into an Ident.
///
/// #### Returns
///
/// The Ident representation of the string.
pub fn str_to_ident(string: String) -> Ident {
    Ident::new(&string, proc_macro::Span::call_site().into())
}
