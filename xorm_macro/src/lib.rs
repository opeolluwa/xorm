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

// log error to stdout
// TODO: fix this
// macro_rules! bail {( $err_msg:expr$(, $span:expr)? $(,)? ) => (
//     {
//         let mut _span = ::proc_macro::Span::call_site();
//         $( _span = $span; )?
//         return ::syn::Error::new(_span, $err_msg)
//                    .to_compile_error()
//                    .into()
//         ;
//     }
// )}
// use syn::spanned::Spanned;

#[proc_macro_derive(IntoModel, attributes(table_name))]
pub fn find_by_pk_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_macro(&ast)
}

/// the macro implementation
fn impl_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    // get the name of the struct on which the trait is implemented and the table name
    let name = &ast.ident;
    let table_name_attribute = &ast
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident("table_name"));

    // if the table name is  not provide log error thru the bail macro
    // bail if it doesn't exist
    // TODO: implement proper error handling
    if table_name_attribute.is_none() {
        panic!(
            "missing table name attribute,\nPlease add table name\ne.g. #[table_name = user_information]",
            // &ast.ident.span()
        )
    }

    // let table_name = table_name_attribute.unwrap().parse_meta();

    let tbl_name = match table_name_attribute.unwrap().parse_meta() {
        Ok(syn::Meta::NameValue(value)) => {
            if let syn::Lit::Str(table_name) = value.lit {
                table_name
            } else {
                todo!()
                // println!("expected a float value, e.g. #[orbital_period = 1.0]");
            }
        }
        Ok(syn::Meta::Path(_)) | Ok(syn::Meta::List(_)) => todo!(),
        Err(err_msg) => {
            panic!("an error occurred due to >{err_msg}")
            // todo!()
        }
    };
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
            // const TABLE_NAME :String = #tbl_name;
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

/*   Ok(/*bad*/ _) => panic!(
    "expected a NameValue style attribue, e.g. #[orbital_period = 1.0]",
    // bad.path().span()
), */

// Let's assume that we do not want to accept enums as input to our custom derive method.

// This condition can be easily checked with the help of syn. But how do we tell the user, that we do not accept enums? The idiomatic way to report errors in procedural macros is to panic:

// fn impl_hello_world(ast: &syn::DeriveInput) -> quote::Tokens {
//     let name = &ast.ident;
//     // Check if derive(HelloWorld) was specified for a struct
//     if let syn::Body::Struct(_) = ast.body {
//         // Yes, this is a struct
//         quote! {
//             impl HelloWorld for #name {
//                 fn hello_world() {
//                     println!("Hello, World! My name is {}", stringify!(#name));
//                 }
//             }
//         }
//     } else {
//         //Nope. This is an Enum. We cannot handle these!
//        panic!("#[derive(HelloWorld)] is only defined for structs, not for enums!");
//     }
// }
