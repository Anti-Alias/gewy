use proc_macro::{TokenStream};
use syn::{Data, DataStruct, DeriveInput, Field};

#[proc_macro_derive(State, attributes(child, value))]
pub fn derive_state(stream: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(stream as DeriveInput);
    let Data::Struct(data) = input.data else { panic!("Only structs supported") };
    let code = format!(
        "impl gewy::State for {} {{ {} }}",
        input.ident,
        generate_body(&data),
    );
    code.parse().unwrap()
}


fn generate_body(data: &DataStruct) -> String {
    let fields = get_fields(&data);
    let value_field = fields
        .iter()
        .find(|field| field.attributes == Attributes::Value)
        .expect("Value field missing");
    let value_fn = format!("fn value(&self) -> &dyn std::any::Any {{ &self.{} }}", value_field.name);
    let child_fn = generate_child_fn(&fields);
    let child_mut_fn = generate_child_fn_mut(&fields);
    format!("{} {} {}", value_fn, child_fn, child_mut_fn)
}

fn generate_child_fn(fields: &[FieldWithAttrs]) -> String {
    let mut code = String::new();
    code.push_str("fn child<'a>(&'a self, name: &str) -> Option<&'a dyn std::any::Any> { ");
    code.push_str("match name { ");
    for field in fields {
        let Attributes::Child { name } = &field.attributes else { continue };
        code.push_str("\"");
        code.push_str(&name.to_string());
        code.push_str("\" => Some(&self.");
        code.push_str(&field.name.to_string());
        code.push_str("),");
    }
    code.push_str("_ => None,");
    code.push_str("}");
    code.push_str("}");
    code
}

fn generate_child_fn_mut(fields: &[FieldWithAttrs]) -> String {
    let mut code = String::new();
    code.push_str("fn child_mut<'a>(&'a mut self, name: &str) -> Option<&'a mut dyn std::any::Any> { ");
    code.push_str("match name { ");
    for field in fields {
        let Attributes::Child { name } = &field.attributes else { continue };
        code.push_str("\"");
        code.push_str(&name.to_string());
        code.push_str("\" => Some(&mut self.");
        code.push_str(&field.name.to_string());
        code.push_str("),");
    }
    code.push_str("_ => None,");
    code.push_str("}");
    code.push_str("}");
    code
}

fn get_value_field(data: &DataStruct) -> String {
    for (i, field) in data.fields.iter().enumerate() {
        let field_name = match &field.ident {
            Some(ident) => ident.to_string(),
            None => i.to_string(),
        };
    }
    panic!("Value field not found");
}

fn get_fields(data: &DataStruct) -> Vec<FieldWithAttrs> {
    data.fields
        .iter()
        .enumerate()
        .flat_map(|(i, field)| FieldWithAttrs::from_field(i, field))
        .collect()
}

/// A struct field with metadata
struct FieldWithAttrs {
    name: String,
    attributes: Attributes,
} 

impl FieldWithAttrs {
    fn from_field(index: usize, field: &Field) -> Option<Self> {
        let field_name = match &field.ident {
            Some(ident) => ident.to_string(),
            None => index.to_string(),
        };
        for attr in &field.attrs {
            for segment in attr.path().segments.iter() {
                let attr_name = segment.ident.to_string();
                match attr_name.as_str() {
                    "child" => return Some(Self {
                        name: field_name.clone(),
                        attributes: Attributes::Child {
                            name: field_name.clone()
                        },
                    }),
                    "value" => return Some(Self {
                        name: field_name.clone(),
                        attributes: Attributes::Value,
                    }),
                    _ => {},
                }
            }
        }
        None
    }
}

#[derive(Eq, PartialEq)]
enum Attributes {
    Value,
    Child { name: String },
}

