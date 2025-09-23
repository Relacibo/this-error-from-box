//! Procedural macro for automatic From implementation for #[from] Box<T>

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data, DeriveInput, Fields, Ident, Path,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

struct WrapperArg {
    wrapper: Option<syn::Path>,
}

impl Parse for WrapperArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            Ok(WrapperArg { wrapper: None })
        } else {
            let wrapper: syn::Path = input.parse()?;
            Ok(WrapperArg {
                wrapper: Some(wrapper),
            })
        }
    }
}

#[proc_macro_attribute]
pub fn this_error_from_box(attr: TokenStream, item: TokenStream) -> TokenStream {
    let WrapperArg { wrapper } = parse_macro_input!(attr as WrapperArg);
    let wrapper_ident = wrapper.unwrap_or_else(|| syn::parse_str("Box").unwrap());
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

            let Some(last_segment) = type_path.path.segments.last() else {
                continue;
            };

            if type_path.path.leading_colon.is_some() ^ wrapper_ident.leading_colon.is_some() {
                continue;
            }

            if type_path.path.segments.len() != wrapper_ident.segments.len() {
                continue;
            }
            
            let paths_equal = type_path
                .path
                .segments
                .iter()
                .zip(wrapper_ident.segments.iter())
                .all(|(a, b)| a.ident == b.ident);

            if !paths_equal {
                continue;
            }

            let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments else {
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
                        #enum_name::#variant_ident(#wrapper_ident::from(e))
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
