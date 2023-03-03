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

#[proc_macro_derive(IntoModel)]
pub fn find_by_pk_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_macro(&ast)
}

/// the macro implementation
fn impl_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    /* get the name of the struct on which the trait is implemented
     // # example
     #[derive(IntoModel)]
    struct UserInformation{
    some fields here
    }
    The name filed below will return `UserInformation`
    */
    let name = &ast.ident;

    // get the names of the fields available on the struct that implement the trait
    let fields_named = {
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

        // return the named fields
        &fields_named
            .ok_or("Unable to retrieve fields named")
            .unwrap()
            .named
    };

    // get the fields separate by a comma
    // e.g SELECT name, age, gender ...
    // where name age, gender are the fields
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

    // generate query to  fetch py pk
    //TODO: make it possible for types that implement into model trait
    let find_by_pk_query = { format!("SELECT {} FROM {}", fields, name) };

    // delete statement
    let destroy_query = { format!("DELETE FROM {} WHERE $CONDITION", name) };

    // find or create statement
    let find_or_create_query = {
        format!(
            "INSERT INTO {table} SELECT {fields} FROM ( SELECT * FROM {table} WHERE {condition})",
            table = name,
            fields = &fields,
            condition = "$UNIQUE_KEY"
        )
    };
    // the actual implementation of the traits
    let gen = quote::quote! {
        impl IntoModel for #name {
            // find record by primary key
            fn find_by_pk() {
                println!("{}", #find_by_pk_query);
            }

        //  find or create record
        fn find_or_create(){
        println!("{}", #find_or_create_query);
            }

        // delete record
         fn destroy(){
        println!(" delete a record {}", #destroy_query);
            }
        }
    };
    gen.into()
}
