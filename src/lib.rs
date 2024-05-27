extern crate proc_macro;

use std::collections::HashMap;

use proc_macro::{Group, Ident, TokenStream, TokenTree};

#[proc_macro]
pub fn general_structs(item: TokenStream) -> TokenStream {
    let mut types_structs = Vec::<Ident>::new();
    let mut general: Group = Group::new(proc_macro::Delimiter::Bracket, TokenStream::new());
    let mut features_structs: HashMap<String, Group> = HashMap::new();
    let mut current_type: String = String::new();
    let mut is_types = false;
    let mut is_general = false;
    let mut is_features = false;

    for token in item.into_iter() {
        // structs
        if let TokenTree::Ident(ref ident) = token {
            if ident.to_string() == "structs".to_string() {
                is_types = true;
                continue;
            }
        }

        if let TokenTree::Punct(ref ch) = token {
            if is_types {
                is_types = !(ch.as_char() == ';');
            }
        }

        if is_types {
            if let TokenTree::Ident(ref ident) = token {
                types_structs.push(ident.clone());
            }
        }

        // general
        if let TokenTree::Ident(ref ident) = token {
            if ident.to_string() == "general".to_string() {
                is_general = true;
                continue;
            }
        }

        if let TokenTree::Ident(ref ident) = token {
            if is_general {
                is_general = !(ident.to_string() == "features".to_string());
            }
        }

        if is_general {
            if let TokenTree::Group(ref group) = token {
                general = group.clone();
            }
        }

        // features
        if let TokenTree::Ident(ref ident) = token {
            if ident.to_string() == "features".to_string() {
                is_features = true;
                continue;
            }
        }

        if is_features {
            if let TokenTree::Ident(ref ident) = token {
                current_type = ident.to_string();
            }

            if let TokenTree::Group(ref group) = token {
                features_structs.insert(current_type.clone(), group.clone());
            }
        }
    }

    let mut res_str_structs = String::new();
    for ident in types_structs.iter() {
        let mut structure = format!(
            "struct {}",
            ident.to_string()
        );

        let group = general.stream().to_string();
        structure.push_str(format!("{{ {}", group.clone()).as_str());


        if let Some(group) = features_structs.get(&ident.clone().to_string()) {
            let group = group.stream().to_string();
            structure.push_str(format!("{}", group.clone()).as_str());
        }

        structure.push_str(" }");

        res_str_structs.push_str(structure.as_str());
    }

    
    res_str_structs.as_str().parse().unwrap()
}
