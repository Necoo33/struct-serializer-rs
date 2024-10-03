extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(SerializeStruct)]
pub fn serialize_struct_inner(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone(); // Struct ismini alıyoruz

    let fields = if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            fields.named
        } else {
            panic!("IterateFields can only be used with named fields");
        }
    } else {
        panic!("IterateFields can only be used with structs");
    };

    let field_count = fields.len(); // Alan sayısını alıyoruz
    let field_access: Vec<_> = fields.iter().map(|f| {
        let name = &f.ident;
        let name_str = name.as_ref().unwrap().to_string();
        let ty = &f.ty;
        quote! {
            (String::from(#name_str), format!("{}", &self.#name), String::from(stringify!(#ty)))
        }
    }).collect();

    let expanded = quote! {
        impl SerializeStruct for #name {
            fn serialize_struct_inner(&self) -> Vec<(String, String, String)> {
                let mut fields = vec![
                    (String::from(stringify!(#name)), #field_count.to_string(), "descriptor".to_string())
                ];
                fields.extend(vec![#(#field_access),*]);
                fields
            }
        }
    };

    TokenStream::from(expanded)
}
