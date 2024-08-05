use proc_macro2::Span;

pub fn non_enum_error() -> syn::Error {
    syn::Error::new(Span::call_site(), "This macro only supports enums.")
}

pub fn non_serde_rename_error() -> syn::Error {
    syn::Error::new(
        Span::call_site(),
        "This macro only supports enums with serde attributes and rename fields.",
    )
}
