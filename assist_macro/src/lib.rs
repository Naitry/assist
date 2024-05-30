extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use syn::{parse_macro_input, Attribute, ItemFn};

#[derive(Serialize)]
struct Parameter {
    #[serde(rename = "type")]
    param_type: String,
    description: String,
}

#[derive(Serialize)]
struct Parameters {
    #[serde(rename = "type")]
    param_type: String,
    properties: HashMap<String, Parameter>,
    required: Vec<String>,
}

#[derive(Serialize)]
struct FunctionDescription {
    name: String,
    description: String,
    parameters: Parameters,
}

fn extract_docstring(attrs: &[Attribute]) -> String {
    let mut docstring = String::new();
    for attr in attrs {
        if attr.path.is_ident("doc") {
            if let Ok(meta) = attr.parse_meta() {
                if let syn::Meta::NameValue(nv) = meta {
                    if let syn::Lit::Str(lit_str) = nv.lit {
                        docstring.push_str(&lit_str.value());
                        docstring.push('\n');
                    }
                }
            }
        }
    }
    docstring.trim().to_string()
}

fn parse_docstring(docstring: &str) -> (String, HashMap<String, String>) {
    let mut description = String::new();
    let mut parameters = HashMap::new();
    let mut lines = docstring.lines();

    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed.starts_with(":param") {
            let parts: Vec<&str> = trimmed.splitn(3, ' ').collect();
            if parts.len() == 3 {
                let param_name = parts[1].trim_end_matches(':').trim();
                let param_description = parts[2].trim();
                parameters.insert(param_name.to_string(), param_description.to_string());
            }
        } else if !trimmed.starts_with(":") {
            description.push_str(trimmed);
            description.push('\n');
        }
    }

    (description.trim().to_string(), parameters)
}

#[proc_macro_attribute]
pub fn generate_tools_config(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let function_name = input.sig.ident.to_string();
    let docstring = extract_docstring(&input.attrs);
    let (description, doc_parameters) = parse_docstring(&docstring);

    let mut properties = HashMap::new();
    let mut required = vec![];

    for input in input.sig.inputs.iter() {
        if let syn::FnArg::Typed(pat_type) = input {
            if let syn::Pat::Ident(ident) = &*pat_type.pat {
                let name = ident.ident.to_string();
                let param_type = match &*pat_type.ty {
                    syn::Type::Path(type_path) => type_path.path.segments.last().unwrap().ident.to_string(),
                    _ => "unknown".to_string(),
                };

                let param_name_without_underscore = name.trim_start_matches('_');
                let param_description = doc_parameters.get(param_name_without_underscore).cloned().unwrap_or_default();

                properties.insert(
                    name.clone(),
                    Parameter {
                        param_type,
                        description: param_description,
                    },
                );
                required.push(name);
            }
        }
    }

    let parameters = Parameters {
        param_type: "object".to_string(),
        properties,
        required,
    };

    let function_description = FunctionDescription {
        name: function_name,
        description,
        parameters,
    };

    let json_output = serde_json::to_string_pretty(&function_description).unwrap();

    // Write JSON to a file
    let mut file = File::create("function_description.json").expect("Unable to create file");
    file.write_all(json_output.as_bytes()).expect("Unable to write data");

    let expanded = quote! {
        #input
    };

    TokenStream::from(expanded)
}

