use syn::{Data, Field};

pub(crate) fn parse_macro_data(data: Data) -> Vec<Field> {
    let raw_fields = match data {
        syn::Data::Struct(d) => d.fields,
        _ => panic!("Only structs are supported"),
    };
    raw_fields.iter().cloned().collect::<Vec<Field>>()
}
