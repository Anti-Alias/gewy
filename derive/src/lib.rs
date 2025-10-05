use proc_macro::{TokenStream};
use syn::{Data, DeriveInput, Field, Ident};

#[proc_macro_derive(State, attributes(state))]
pub fn derive_state(stream: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(stream as DeriveInput);
    let code = format!(
        "impl gewy::State for {} {{ {} }}",
        input.ident,
        &generate_body(input.data)
    );
    code.parse().unwrap()
}


fn generate_body(data: Data) -> String {
    let field_defs = get_field_defs(data);
    let state_fn = generate_state_fn(&field_defs);
    let state_mut_fn = generate_state_mut_fn(&field_defs);
    let as_any_fn = "fn as_any_ref(&self) -> &dyn std::any::Any { self }";
    let as_any_mut_fn = "fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }";
    format!("{} {} {} {}", state_fn, state_mut_fn, as_any_fn, as_any_mut_fn)
}

fn generate_state_fn(field_defs: &[FieldDef]) -> String {
    if field_defs.is_empty() {
        return String::new();
    }
    let mut code = String::new();
    code.push_str("fn state<'a>(&'a self, name: &str) -> Option<&'a dyn gewy::State> { ");
    code.push_str("match name { ");
    for field in field_defs {
        code.push_str("\"");
        code.push_str(&field.name.to_string());
        code.push_str("\" => Some(&self.");
        code.push_str(&field.field_name.to_string());
        code.push_str("),");
    }
    code.push_str("_ => None,");
    code.push_str("}");
    code.push_str("}");
    code
}

fn generate_state_mut_fn(field_defs: &[FieldDef]) -> String {
    if field_defs.is_empty() {
        return String::new();
    }
    let mut code = String::new();
    code.push_str("fn state_mut<'a>(&'a mut self, name: &str) -> Option<&'a mut dyn gewy::State> { ");
    code.push_str("match name { ");
    for field in field_defs {
        code.push_str("\"");
        code.push_str(&field.name.to_string());
        code.push_str("\" => Some(&mut self.");
        code.push_str(&field.field_name.to_string());
        code.push_str("),");
    }
    code.push_str("_ => None,");
    code.push_str("}");
    code.push_str("}");
    code
}

fn get_field_defs(data: Data) -> Vec<FieldDef> {
    let Data::Struct(data) = data else {
        panic!("Only structs supported")
    };
    data.fields
        .into_iter()
        .enumerate()
        .flat_map(|(i, field)| FieldDef::from_field(i, field))
        .collect()
}

#[derive(Debug)]
struct FieldDef {
    field_name: String,
    name: String,
}


impl FieldDef {
    fn from_field(index: usize, field: Field) -> Option<Self> {
        let field_name = match field.ident {
            Some(ident) => ident.to_string(),
            None => index.to_string(),
        };
        for attr in field.attrs {
            for segment in attr.path().segments.iter() {
                if segment.ident.to_string() == "state" {
                    return Some(Self {
                        field_name: field_name.clone(),
                        name: field_name,
                    })
                }
            }
        }
        None
    }
}

