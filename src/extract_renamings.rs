use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, Data, Lit, Meta, MetaNameValue, Token};
use syn::{DeriveInput, Expr};

use crate::utils::{non_enum_error, non_serde_rename_error};

pub(crate) fn derive_all_variants_renamings_inner(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_identifier = &input.ident;
    let mut implementation = quote! {
        let mut variants: Vec<String> = Vec::new();
    };

    // search for only inputs that are enums
    match input.data {
        Data::Enum(d) => {
            // iterate over variants that have attributes serde, if there is no serde attribute with rename field, return an error
            for v in d.variants.iter() {
                let mut renamed_variant_name = Option::<String>::None;
                for attr in v.attrs.iter() {
                    if attr.path.is_ident("serde") {
                        let nested =
                            attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
                        for meta in nested {
                            match meta {
                                Meta::NameValue(MetaNameValue {
                                    path,
                                    lit: Lit::Str(string_literal),
                                    ..
                                }) if path.is_ident("rename") => {
                                    renamed_variant_name = Some(string_literal.value());
                                }
                                _ => {}
                            }
                        }
                    }

                    if let Some(ref variant_name) = renamed_variant_name {
                        implementation.extend(quote! {
                            variants.push(#variant_name.to_string());
                        });
                    } else {
                        return Err(non_serde_rename_error());
                    }
                }
            }
        }
        _ => return Err(non_enum_error()),
    }

    Ok(quote! {impl #struct_identifier {
        pub fn all_variants_serde_renames() -> Vec<String>{
            #implementation
            variants
        }
    }}
    .into())
}

pub(crate) fn derive_variant_name_inner(input: DeriveInput) -> syn::Result<TokenStream> {
    let enum_identifier = &input.ident;
    let mut variants_cases = Vec::new();
    let mut variants_names = Vec::new();

    match input.data {
        Data::Enum(d) => {
            // iterate over variants that have attributes serde, if there is no serde attribute with rename field, return an error
            for v in d.variants.iter() {
                let mut renamed_variant_name = Option::<String>::None;
                for attr in v.attrs.iter() {
                    if attr.path.is_ident("serde") {
                        let nested =
                            attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
                        for meta in nested {
                            match meta {
                                Meta::NameValue(MetaNameValue {
                                    path,
                                    lit: Lit::Str(string_literal),
                                    ..
                                }) if path.is_ident("rename") => {
                                    renamed_variant_name = Some(string_literal.value());
                                }
                                _ => {}
                            }
                        }
                    }

                    if let Some(ref variant_name) = renamed_variant_name {
                        match v.fields {
                            syn::Fields::Unit => variants_cases.push(v.ident.to_token_stream()),
                            _ => {
                                variants_cases.push(
                                    syn::parse_str::<Expr>(&format!("{}(_)", v.ident))?
                                        .to_token_stream(),
                                );
                            }
                        }
                        variants_names.push(variant_name.clone());
                    } else {
                        return Err(non_serde_rename_error());
                    }
                }
            }
        }
        _ => return Err(non_enum_error()),
    }
    let expanded = quote! {
        impl #enum_identifier {
            pub fn variant_name(&self) -> &str {
                match self {
                    #(#enum_identifier::#variants_cases => #variants_names),*
                }
            }
        }
    };
    Ok(expanded.into())
}
