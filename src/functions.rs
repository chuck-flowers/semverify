use serde::Deserialize;
use serde::Serialize;
use syn::ItemFn;

/// Describes a Rust function
#[derive(Deserialize, Serialize)]
pub struct FunctionMetadata {
    name: String,
}

impl From<ItemFn> for FunctionMetadata {
    fn from(src: ItemFn) -> Self {
        let name = src.sig.ident.to_string();
        Self { name }
    }
}
