use proc_macro2::TokenStream;
use quote::quote;
use syn;


// the impl macro body
pub fn impl_macro(ast: &syn::DeriveInput) -> TokenStream {
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
