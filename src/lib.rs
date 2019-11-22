extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, Fields, ItemEnum, Token};

#[proc_macro_attribute]
pub fn first_class_variants(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemEnum);
    let name = &input.ident;
    let attrs = &input.attrs;
    let variants = &input.variants;

    let make_struct_ident = |variant_ident: &Ident| {
        Ident::new(
            &format!("{}{}", name.to_string(), variant_ident.to_string()),
            Span::call_site(),
        )
    };
    let variant_structs = variants.iter().map(|v| {
        let variant_ident = &v.ident;
        let struct_ident = make_struct_ident(variant_ident);
        let fields = &v.fields;
        let semicolon = match &v.fields {
            Fields::Named(_) => None,
            _ => Some(<Token!(;)>::default()),
        };
        quote! {
            #(#attrs)*
            pub struct #struct_ident #fields #semicolon
            impl Into<#name> for #struct_ident {
                fn into(self) -> #name {
                    #name::#variant_ident(self)
                }
            }
        }
    });
    let wrapper_variants = variants.iter().map(|v| {
        let variant_ident = &v.ident;
        let struct_ident = make_struct_ident(variant_ident);
        quote! {
            #variant_ident(#struct_ident)
        }
    });
    let result = quote! {
        enum #name {
            #(#wrapper_variants,)*
        }
        #(#variant_structs)*
    };
    result.into()
}
