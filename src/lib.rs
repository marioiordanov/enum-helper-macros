extern crate proc_macro;

use extract_renamings::{derive_all_variants_renamings_inner, derive_variant_name_inner};
use proc_macro::TokenStream;

mod extract_renamings;
mod utils;

#[proc_macro_derive(AllVariantsSerdeRenames)]
pub fn extract_renamed_names(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    derive_all_variants_renamings_inner(input).unwrap_or_else(|err| err.to_compile_error().into())
}

#[proc_macro_derive(VariantName)]
pub fn derive_variant_name(item: TokenStream) -> TokenStream {
    let syn_item: syn::DeriveInput = syn::parse_macro_input!(item as syn::DeriveInput);

    derive_variant_name_inner(syn_item).unwrap_or_else(|err| err.to_compile_error().into())
}
