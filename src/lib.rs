// Copyright 2024 Dmitriy Mayorov

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
        // types
        if let TokenTree::Ident(ref ident) = token {
            if ident.to_string() == "types".to_string() {
                is_types = true;
                continue;
            }
        }

        if let TokenTree::Ident(ref ident) = token {
            if is_types {
                is_types = !(ident.to_string() == "general".to_string());
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
        let mut structure = format!("struct {}", ident.to_string());

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
