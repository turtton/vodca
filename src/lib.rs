use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed};

#[proc_macro_derive(Fromln)]
pub fn derive_fromln(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let (impl_generics, type_generics, _) = ast.generics.split_for_impl();

    let field_type = get_field_type(&ast.data);
    quote! {
        impl #impl_generics From<#name #type_generics> for #field_type {
            fn from(value: #name #type_generics) -> Self {
                value.0
            }
        }
    }
    .into()
}

#[proc_macro_derive(AsRefln)]
pub fn derive_asrefln(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let (impl_generics, type_generics, _) = ast.generics.split_for_impl();
    let field_type = get_field_type(&ast.data);

    quote! {
        impl #impl_generics AsRef<#field_type> for #name #type_generics {
            fn as_ref(&self) -> &#field_type {
                &self.0
            }
        }
    }
    .into()
}

fn get_field_type(data: &Data) -> proc_macro2::TokenStream {
    if let Data::Struct(DataStruct {
        fields: Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }),
        ..
    }) = data
    {
        let ty = &unnamed
            .first()
            .expect("Tuple struct must have at least one field")
            .ty;
        quote! {
            #ty
        }
    } else {
        unimplemented!("Only tuple structs are supported")
    }
}

#[proc_macro_derive(References)]
pub fn derive_references(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let generics = &ast.generics;

    let fields = if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!("Only normal structs are supported")
    };

    let field_refs = fields.iter().map(|field| {
        let field_name = &field.ident;
        let ty = &field.ty;
        quote! {
            pub fn #field_name(&self) -> &#ty {
                &self.#field_name
            }
        }
    });
    quote! {
        impl #generics #name #generics {
            #(#field_refs)*
        }
    }
    .into()
}
