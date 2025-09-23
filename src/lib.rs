//! Procedural macro for automatic From implementation for #[from] Box<T>

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::{Parse, ParseStream}, parse_macro_input, Data, DeriveInput, Fields, Ident, Path};

struct WrapperArg {
    wrapper: Option<Ident>,
}

impl Parse for WrapperArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            Ok(WrapperArg { wrapper: None })
        } else {
            let wrapper: syn::Ident = input.parse()?;
            Ok(WrapperArg { wrapper: Some(wrapper) })
        }
    }
}

#[proc_macro_attribute]
pub fn this_error_from_box(attr: TokenStream, item: TokenStream) -> TokenStream {
    let WrapperArg { wrapper } = parse_macro_input!(attr as WrapperArg);
    let wrapper_ident = wrapper
        .as_ref()
        .map(|w| w.to_string())
        .unwrap_or_else(|| "Box".to_string());
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
            let segments = &type_path.path.segments;
            let last_segment = match segments.last() {
                Some(seg) => seg,
                None => continue,
            };
            if last_segment.ident != wrapper_ident {
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
            let wrapper_path: Path = syn::parse_str(&wrapper_ident).unwrap();
            from_impls.push(quote! {
                impl ::std::convert::From<#inner_ty> for #enum_name {
                    fn from(e: #inner_ty) -> Self {
                        #enum_name::#variant_ident(#wrapper_path::from(e))
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
