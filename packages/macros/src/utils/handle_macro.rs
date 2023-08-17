use syn::{DeriveInput, Ident, Type};

pub fn handle_macro(input: DeriveInput) -> (Ident, Vec<Type>, Vec<Ident>) {
    let DeriveInput { ident, data, .. } = input;
    let name = ident;
    let raw_fields = match data {
        syn::Data::Struct(d) => d.fields,
        _ => panic!("Only structs are supported"),
    };
    let types = raw_fields
        .iter()
        .map(|f| f.ty.clone())
        .collect::<Vec<Type>>();
    let keys = raw_fields
        .iter()
        .map(|f| f.ident.clone().unwrap())
        .collect::<Vec<Ident>>();
    (name, types, keys)
}
