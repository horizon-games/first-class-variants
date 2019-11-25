extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, AttributeArgs, Field, Fields, ItemEnum, Token,
    VisPublic,
};

#[proc_macro_attribute]
pub fn first_class_variants(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemEnum);
    let vis = &input.vis;
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
        let mut fields = v.fields.clone();
        match &mut fields {
            Fields::Named(named) => make_pub(&mut named.named),
            Fields::Unnamed(unnamed) => make_pub(&mut unnamed.unnamed),
            _ => {}
        }

        let semicolon = match &v.fields {
            Fields::Named(_) => None,
            _ => Some(<Token!(;)>::default()),
        };
        quote! {
            #(
                #[#attr_args]
            )*
            pub struct #struct_ident #fields #semicolon
            impl From<#struct_ident> for #name {
                fn from(subtype_struct: #struct_ident) -> Self {
                    #name::#variant_ident(subtype_struct)
                }
            }
            impl std::convert::TryFrom<#name> for #struct_ident {
                type Error = (); // There's only one possible error - enum variant isn't this struct.
                fn try_from(enum_variant: #name) -> Result<Self, Self::Error> {
                    match enum_variant {
                        #name::#variant_ident(subtype_struct) => Ok(subtype_struct),
                        _ => Err(())
                    }
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
        #(#attrs)*
        #vis enum #name {
            #(#wrapper_variants,)*
        }
        #(#variant_structs)*
    };
    result.into()
}

fn make_pub(punctuated: &mut Punctuated<Field, Token![,]>) {
    for field in punctuated.iter_mut() {
        field.vis = VisPublic {
            pub_token: <Token![pub]>::default(),
        }
        .into();
    }
}
