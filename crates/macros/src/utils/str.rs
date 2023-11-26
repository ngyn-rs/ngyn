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

/// Generate a random str and attach to prefix using std only and not using rand
pub fn random_str_from(prefix: String) -> String {
    let suffix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
        .to_string();
    format!("{}{}", prefix, suffix)
}
