extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token, Field, Fields, FieldsUnnamed, ItemEnum,
    Token, Variant, Visibility,
};

#[proc_macro_attribute]
pub fn first_class_variants(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemEnum);
    let name = &input.ident;
    let attrs = &input.attrs;
    let variants = &input.variants;
    let variant_structs = variants.iter().map(|v| {
        let ident = &v.ident;
        let fields = &v.fields;
        let struct_def = match &v.fields {
            Fields::Named(_) => {
                quote! {
                    pub struct #ident #fields
                }
            }
            _ => {
                quote! {
                    pub struct #ident #fields;
                }
            }
        };
        quote! {
            #(#attrs)*
            #struct_def
            impl Into<#name> for #ident {
                fn into(self) -> #name {
                    #name::#ident(self)
                }
            }
        }
    });
    let wrapper_variants = variants.iter().map(|v| {
        let ident = &v.ident;
        quote! {
            #ident(#ident)
        }
    });
    let result = quote! {
        enum #name {
            #(#wrapper_variants,)*
        }
        #(#variant_structs)*
    };
    dbg!(quote! { #input }.to_string());
    dbg!(&result.to_string());
    result.into()
}
