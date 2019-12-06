use serde::Deserialize;
use serde::Serialize;
use syn::ItemEnum;

/// Describes a Rust enum
#[derive(Deserialize, Serialize)]
pub struct EnumMetadata {
    name: String,
}

impl From<ItemEnum> for EnumMetadata {
    fn from(src: ItemEnum) -> Self {
        let name = src.ident.to_string();
        Self { name }
    }
}
