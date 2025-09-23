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
            if let Fields::Unnamed(fields) = &variant.fields {
                if fields.unnamed.len() == 1 {
                    let field = &fields.unnamed[0];
                    let mut has_from = false;
                    for attr in &field.attrs {
                        if attr.path().is_ident("from") {
                            has_from = true;
                            break;
                        }
                    }
                    if has_from {
                        // Check if type is Box<T>
                        if let syn::Type::Path(type_path) = &field.ty {
                            let segments = &type_path.path.segments;
                            // Check if the path is really std::boxed::Box or alloc::boxed::Box
                            let is_std_box = {
                                let segs: Vec<_> = type_path.path.segments.iter().map(|s| s.ident.to_string()).collect();
                                (segs == ["std", "boxed", "Box"])
                                    || (segs == ["alloc", "boxed", "Box"])
                                    || (segs == ["Box"]) // fallback for just Box
                            };
                            if is_std_box {
                                if let syn::PathArguments::AngleBracketed(args) = &segments.last().unwrap().arguments {
                                    if args.args.len() == 1 {
                                        if let syn::GenericArgument::Type(inner_ty) = &args.args[0] {
                                            let variant_ident = &variant.ident;
                                            from_impls.push(quote! {
                                                impl ::std::convert::From<#inner_ty> for #enum_name {
                                                    fn from(e: #inner_ty) -> Self {
                                                        #enum_name::#variant_ident(::std::boxed::Box::new(e))
                                                    }
                                                }
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let expanded = quote! {
        #input
        #(#from_impls)*
    };
    TokenStream::from(expanded)
}
