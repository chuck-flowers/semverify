use serde::Deserialize;
use serde::Serialize;
use syn::ItemConst;

/// Describes a Rust const
#[derive(Deserialize, Serialize)]
pub struct ConstMetadata {
    name: String,
}

impl From<ItemConst> for ConstMetadata {
    fn from(src: syn::ItemConst) -> Self {
        let name = src.ident.to_string();

        Self { name }
    }
}
