# Helper derive macros for enums

 - AllVariantsSerdeRenames: creates a method `fn all_variants_serde_renames() -> Vec<String>`, returning all variants serde rename attributes data.
 - VariantName: crates a method `fn variant_name(&self) -> &str`, returning the serde rename attribute data.

## NOTE!
Both macros work only if there is `#[serde(rename=...)]` attribute applied to all of the enum variants
