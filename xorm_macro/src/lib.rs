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

/// the macro implementation
fn impl_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    // get the name of the struct that implement the trait  TODO: use an attribute to define the table name
    let name = &ast.ident;

    // get the fields
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
        let fields_named = &fields_named
            .ok_or("Unable to retrieve fields named")
            .unwrap()
            .named;

        fields_named
    };

    //TODO: make it possible for types that implement into model trait
    let find_by_pk_query = {
        format!(
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
        )
    };

    // delete statement
    let destroy_query = { format!("DELETE FROM {} WHERE $CONDITION", name) };

    // find or create statement
    let find_or_create_query = {
        format!(
            "INSERT INTO {table} SELECT {fields:?} FROM ( SELECT * FROM {table} WHERE {condition})",
            table = name,
            fields = fields_named
                .iter()
                .map(|f| format!("{}", f.ident.as_ref().unwrap()))
                .fold(String::default(), |a, f| if a.is_empty() {
                    f
                } else {
                    a + ", " + f.as_str()
                }),
            condition = "$UNIQUE_KEY"
        )
    };

    let create_record_query = {
        format!(
            "INSERT {fields} INTO {table} VALUES {values} ",
            table = name,
            fields = fields_named
                .iter()
                .map(|f| format!("{}", f.ident.as_ref().unwrap()))
                .fold(String::default(), |a, f| if a.is_empty() {
                    f
                } else {
                    a + ", " + f.as_str()
                }),
            values = "$VALUES"
        )
    };

    // the actual implementation of the traits
    let gen = quote::quote! {
              #[async_trait::async_trait]

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

              //create record
             async  fn create(){
    println!(" delete a record {}", #create_record_query);
              }
          };
    gen.into()
}

// takes an sql query then returns a result type containing the result and error
// async fn execute_query(query: String) -> Result<(), tokio_postgres::Error> {
//     println!("the query is  {}", query);
//     Ok(())
// }
