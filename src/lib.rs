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

use proc_macro2::{TokenTree, TokenStream, Delimiter, Group};

#[derive(Debug, Clone, Default)]
struct TypeStruct {
    pub signature: Vec<TokenTree>,
    pub group: Vec<TokenTree>, 
}

#[proc_macro]
pub fn general_structs(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = TokenStream::from(item);
    
    let mut metadata = Vec::<TokenTree>::new();
    let mut types = Vec::<TypeStruct>::new();
    let mut general_body = TokenTree::from(Group::new(Delimiter::Brace, TokenStream::new()));
    
    let mut mut_types = Vec::<TypeStruct>::new();
    
    let mut bracket_stack = 0;
    let mut is_struct = false;
    let mut is_mut_struct = false;
    let mut is_body_general_group = false;
    
    let mut current_type_struct = TypeStruct::default();
    for token in ast.into_iter() {
        if let TokenTree::Ident(ident) = &token {
            if ident.to_string() == "struct" {
                is_struct = true;
                continue;
            }
        }
        
        if is_struct {
            if let TokenTree::Punct(punct) = &token {
                if punct.as_char() == '<' {
                    bracket_stack += 1;
                } else if punct.as_char() == '>'{
                    bracket_stack -= 1;
                } else if punct.as_char() == '+' && bracket_stack == 0 {
                    types.push(current_type_struct);
                    current_type_struct = TypeStruct::default();
                    continue;
                }
            } else if let TokenTree::Group(group) = &token {
                if let Delimiter::Brace = group.delimiter() {
                    types.push(current_type_struct);
                    current_type_struct = TypeStruct::default();
                    is_struct = false;
                    is_body_general_group = true;
                }
            }
            
            if is_struct {
                current_type_struct.signature.push(token.clone());
            } else {
                current_type_struct = TypeStruct::default();
            }
        }
        
        if is_body_general_group {
            if let TokenTree::Group(group) = &token {
                general_body = TokenTree::Group(group.clone());
                is_body_general_group = false;
                continue;
            }
        }
        
        if let TokenTree::Ident(_) = &token {
            if !is_struct && !is_body_general_group {
                is_mut_struct = true;
            }
        }
        
        if is_mut_struct {
            if let TokenTree::Punct(punct) = &token {
                if punct.as_char() == '<' {
                    bracket_stack += 1;
                } else if punct.as_char() == '>'{
                    bracket_stack -= 1;
                } else if punct.as_char() == '+' && bracket_stack == 0 {
                    types.push(current_type_struct);
                    current_type_struct = TypeStruct::default();
                    continue;
                }
            } else if let TokenTree::Group(group) = &token {
                if let Delimiter::Brace = group.delimiter() {
                    current_type_struct.group.push(TokenTree::Group(group.clone()));
                    mut_types.push(current_type_struct);
                    
                    current_type_struct = TypeStruct::default();
                    continue;
                }
            }

            current_type_struct.signature.push(token.clone());
        }

        if !is_struct && !is_mut_struct && !is_body_general_group {
            metadata.push(token.clone());
            continue;
        }
    }
    
    let mut res = String::new();
    
    for type_struct in types.iter() {
        res.push_str(format!("{} ", TokenStream::from_iter(metadata.clone())).as_str());
        
        let type_name = TokenStream::from_iter(type_struct.signature.clone()).to_string();
        res.push_str(format!("struct {} {{", type_name).as_str());
        
        if let TokenTree::Group(general_group) = &general_body {
            res.push_str(format!("{}", general_group.stream().to_string()).as_str());
            
            for type_mut_struct in mut_types.iter() {
                let mut_type_name = TokenStream::from_iter(type_mut_struct.signature.clone()).to_string();
                if mut_type_name == type_name {
                    if let TokenTree::Group(add_group) = &type_mut_struct.group[0] {
                        res.push_str(format!("{}", add_group.stream().to_string()).as_str());
                    }
                }
            }
        }

        res.push_str(format!("}} ").as_str());
    }
    
    res.as_str().parse().unwrap()
}
