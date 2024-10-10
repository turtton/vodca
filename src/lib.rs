use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Expr, ExprLit, Fields, FieldsNamed,
    FieldsUnnamed, Lit, Meta,
};

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

#[proc_macro_derive(Newln)]
pub fn derive_newln(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let generics = &ast.generics;

    match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { ref named, .. }),
            ..
        }) => {
            let fields = named;
            let field_args = fields.iter().map(|field| {
                let field_name = &field.ident;
                let ty = &field.ty;
                quote! {
                    #field_name: #ty,
                }
            });
            let field_names = fields.iter().map(|field| {
                let field_name = &field.ident;
                quote! {
                    #field_name,
                }
            });
            quote! {
                impl #generics #name #generics {
                    #[allow(clippy::too_many_arguments)]
                    pub fn new(#(#field_args)*) -> Self {
                        Self {
                            #(#field_names)*
                        }
                    }
                }
            }
            .into()
        }
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }),
            ..
        }) => {
            let field_args = unnamed.iter().enumerate().map(|(i, field)| {
                let field_name = Ident::new(&format!("value_{}", i), Span::call_site());
                let ty = &field.ty;
                quote! {
                    #field_name: impl Into<#ty>,
                }
            });
            let field_names = unnamed.iter().enumerate().map(|(i, _)| {
                let field_name = Ident::new(&format!("value_{}", i), Span::call_site());
                quote! {
                    #field_name.into(),
                }
            });
            quote! {
                impl #generics #name #generics {
                    #[allow(clippy::too_many_arguments)]
                    pub fn new(#(#field_args)*) -> Self {
                        Self(
                            #(#field_names)*
                        )
                    }
                }
            }
            .into()
        }
        _ => unimplemented!("Only normal structs are supported"),
    }
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

#[proc_macro_derive(Nameln, attributes(vodca))]
pub fn derive_name(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let generics = &ast.generics;
    let attrs = ast.attrs;
    let mut snake_case = false;
    let mut prefix: Option<String> = None;
    for attr in &attrs {
        if !attr.path().is_ident("vodca") {
            continue;
        }
        // requires `#[vodca(...)]`
        if let Meta::List(meta) = &attr.meta {
            if meta.tokens.is_empty() {
                continue;
            }
        }

        attr.parse_nested_meta(|meta| {
            // `#[vodca(snake_case)]`
            if meta.path.is_ident("snake_case") {
                snake_case = true;
                return Ok(());
            }
            // `#[vodca(prefix = "...")]`
            if meta.path.is_ident("prefix") {
                let expr: Expr = meta.value()?.parse()?;
                if let Expr::Lit(ExprLit {
                    lit: Lit::Str(value),
                    ..
                }) = expr
                {
                    prefix = Some(value.value());
                    return Ok(());
                }
            }
            Err(meta.error("Unknown vodca attribute"))
        })
        .expect("Failed to parse vodca attribute");
    }
    let names = match ast.data {
        Data::Enum(DataEnum { variants, .. }) => variants.into_iter().map(|variant| {
            let ident = variant.ident;
            let parsed_indent = if snake_case {
                ident
                    .to_string()
                    .chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if i != 0 && c.is_uppercase() {
                            format!("_{}", c.to_lowercase())
                        } else {
                            c.to_lowercase().to_string()
                        }
                    })
                    .collect::<String>()
            } else {
                ident.to_string()
            };

            let variant_name = if let Some(prefix) = &prefix {
                format!("{}_{}", prefix, parsed_indent)
            } else {
                parsed_indent.to_string()
            };
            match variant.fields {
                Fields::Named(_) => {
                    quote! { #name::#ident { .. } => #variant_name }
                }
                Fields::Unnamed(_) => {
                    quote! { #name::#ident(_) => #variant_name }
                }
                Fields::Unit => {
                    quote! { #name::#ident => #variant_name }
                }
            }
        }),
        _ => unimplemented!("Only enums are supported"),
    };
    quote! {
        impl #generics #name #generics {
            pub fn name(&self) -> String {
                let name = match self {
                    #(#names,)*
                };
                name.to_string()
            }
        }
    }
    .into()
}
