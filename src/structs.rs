use serde::Deserialize;
use serde::Serialize;
use syn::ItemStruct;

/// Describes a Rust struct
#[derive(Deserialize, Serialize)]
pub struct StructMetadata {
    name: String,
}

impl From<ItemStruct> for StructMetadata {
    fn from(src: ItemStruct) -> Self {
        let name = src.ident.to_string();

        Self { name }
    }
}
