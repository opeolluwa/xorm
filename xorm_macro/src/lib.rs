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

use proc_macro;
use quote;
use syn;

#[proc_macro_derive(IntoModel)]
pub fn find_by_pk_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_macro(&ast)
}

fn impl_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
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
        .unwrap()
        .named;

    //TODO: make it possible for types that implement into model trait
    let statement = format!(
        "SELECT {} FROM {}",
        fields_named
            .iter()
            .map(|f| format!("{}", f.ident.as_ref().unwrap()))
            .fold(String::default(), |a, f| if a.is_empty() {
                f
            } else {
                a + ", " + f.as_str()
            }),
        name
    );

    ///TODO inject here
    let gen = quote::quote! {
        impl IntoModel for #name {
            fn find_by_pk() {
                println!("{}", #statement);
            }

        fn find_or_create(){
        println!(" find or create {}", #statement);
            }


             fn destroy(){
        println!(" delete a record {}", #statement);
            }
        }
    };
    gen.into()
}
