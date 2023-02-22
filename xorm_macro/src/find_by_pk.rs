use proc_macro::TokenStream;
use quote::quote;
use syn;

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

    let gen = quote! {
        impl IntoModel for #name {
            fn find_by_pk() {
                println!("{}", #statement);
            }
        }
    };
    gen.into()
}

// pub mod destroy;
