// Copyright 2023 The Adeoye Adefemi. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(IntoModel)]
pub fn delete_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_macro(&ast)
}

// the impl macro body
fn impl_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct { fields, .. }) = &ast.data {
        Some(fields)
    } else {
        None
    };

    let fields_named = if let Some(syn::Fields::Named(fields_named)) = fields {
        Some(fields_named)
    } else {
        None
    };
    let fields_named = &fields_named
        .ok_or("Unable to retrieve fields named")
        .expect("Error getting model fields")
        .named;

    // the fields names
    let fields = fields_named
        .iter()
        .map(|f| format!("{}", f.ident.as_ref().unwrap()))
        .fold(String::default(), |a, f| {
            if a.is_empty() {
                f
            } else {
                a + ", " + f.as_str()
            }
        });
    //TODO: make it possible for types that implement into model trait
    let sql_statement = format!("DELETE {} FROM {}", fields, name);

    let gen = quote! {
        impl IntoModel for #name {
            fn delete() {
                println!("{}", #sql_statement);
            }
        }
    };
    gen.into()
}
