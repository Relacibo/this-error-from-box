//! Procedural macro for automatic From implementation for #[from] Box<T>

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_attribute]
pub fn this_error_from_box(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let enum_name = &input.ident;
    let mut from_impls = Vec::new();

    if let Data::Enum(data_enum) = &input.data {
        for variant in &data_enum.variants {
            let Fields::Unnamed(fields) = &variant.fields else {
                continue;
            };

            if fields.unnamed.len() != 1 {
                continue;
            }

            let field = &fields.unnamed[0];
            let has_from = field.attrs.iter().any(|attr| attr.path().is_ident("from"));

            if !has_from {
                continue;
            }

            let syn::Type::Path(type_path) = &field.ty else {
                continue;
            };
            let path = &type_path.path;
            if path.leading_colon.is_some() || path.segments.len() != 1 {
                continue;
            }
            
            let ident = &path.segments[0].ident;

            if ident != "Box" {
                continue;
            }
            let segments = &type_path.path.segments;
            let syn::PathArguments::AngleBracketed(args) = &segments[0].arguments else {
                continue;
            };
            if args.args.len() != 1 {
                continue;
            }
            let syn::GenericArgument::Type(inner_ty) = &args.args[0] else {
                continue;
            };

            let variant_ident = &variant.ident;
            from_impls.push(quote! {
                impl ::std::convert::From<#inner_ty> for #enum_name {
                    fn from(e: #inner_ty) -> Self {
                        #enum_name::#variant_ident(#ident::from(e))
                    }
                }
            });
        }
    }

    let expanded = quote! {
        #input
        #(#from_impls)*
    };
    TokenStream::from(expanded)
}
