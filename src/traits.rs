use serde::Deserialize;
use serde::Serialize;
use syn::ItemTrait;

/// Describes a Rust trait
#[derive(Deserialize, Serialize)]
pub struct TraitMetadata {
    name: String,
}

impl From<ItemTrait> for TraitMetadata {
    fn from(src: syn::ItemTrait) -> Self {
        let name = src.ident.to_string();
        Self { name }
    }
}
