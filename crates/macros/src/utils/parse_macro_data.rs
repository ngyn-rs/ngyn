use syn::{Data, Ident, Type};

pub fn parse_macro_data(data: Data) -> (Vec<Type>, Vec<Ident>) {
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
    (types, keys)
}
