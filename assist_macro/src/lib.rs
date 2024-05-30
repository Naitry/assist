extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use syn::{parse_macro_input, ItemFn};

#[derive(Serialize)]
struct Parameter {
    name: String,
    param_type: String,
    description: String,
}

#[derive(Serialize)]
struct FunctionDescription {
    name: String,
    description: String,
    parameters: Vec<Parameter>,
    required: Vec<String>,
}

#[proc_macro_attribute]
pub fn generate_tools_config(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let function_name = input.sig.ident.to_string();
    let docstring = input.attrs.iter()
        .find(|attr| attr.path.is_ident("doc"))
        .and_then(|attr| attr.tokens.to_string().strip_prefix("=").map(|s| s.trim_matches('"').to_string()))
        .unwrap_or_else(|| "No description provided.".to_string());

    let mut parameters = vec![];
    let mut required = vec![];

    for input in input.sig.inputs.iter() {
        if let syn::FnArg::Typed(pat_type) = input {
            if let syn::Pat::Ident(ident) = &*pat_type.pat {
                let name = ident.ident.to_string();
                let param_type = match &*pat_type.ty {
                    syn::Type::Path(type_path) => type_path.path.segments.last().unwrap().ident.to_string(),
                    _ => "unknown".to_string(),
                };
                let description = ""; // Parse from docstring if available
                parameters.push(Parameter { name: name.clone(), param_type, description: description.to_string() });
                required.push(name);
            }
        }
    }

    let function_description = FunctionDescription {
        name: function_name,
        description: docstring,
        parameters,
        required,
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

